use chrono::{DateTime, Utc};
use derive_more::{Deref, Display};
use notion_client::objects::block::{Block as NotionBlock, BlockType};
use notion_client::objects::parent::Parent;
use serde::{Deserialize, Serialize};

/// The identifier for a Notion `Page`. This exists to distinguish
/// between `Page` and `Block` identifiers at compile-time.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Display, Deref)]
pub struct PageID(String);

impl PageID {
    #[must_use]
    pub fn new(id: String) -> Self {
        PageID(id)
    }
}

/// The identifier for a Notion `Block`. This exists to distinguish
/// between `Block` and `Page` identifiers at compile-time.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Display, Deref)]
pub struct BlockID(String);

impl BlockID {
    #[must_use]
    pub fn new(id: String) -> Self {
        BlockID(id)
    }
}

/// A Block represents a single unit of notetaking, and its structure is heavily borrowed
/// from the Notion API's [Block object](https://developers.notion.com/reference/block).
///
/// A Block is always contained within a Page. Its content is defined by the `block_type` field,
/// but a plain form of the Block's text is stored in the `text` field. Finally, a Block
/// can have children, but if you want to fetch them then you must use the
/// notion::retrieve_all_block_children function with this Block's ID
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Block {
    pub id: BlockID,
    pub page_id: PageID,
    pub block_type: BlockType,
    pub text: String,
    pub creation_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
    pub parent: Option<Parent>,
    pub has_children: bool,
}

impl std::hash::Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.0.hash(state);
    }
}

impl Block {
    #[must_use]
    pub fn from_notion_block(notion_block: NotionBlock, page_id: String) -> Self {
        Block {
            id: BlockID(notion_block.id.unwrap_or_default()),
            // TODO: consider removing this, since it is stored multiple times
            // throughout all the blocks, and we don't need it specifically on a block
            // it's a nice-to-have right now
            page_id: PageID(page_id),
            block_type: notion_block.block_type.clone(),
            // this is where the actual Block data is
            // TODO: instead of losing the data about Notion backlinks, as we're currently doing,
            // we should return a more general object that retains the info about Notion backlinks
            text: notion_block
                .block_type
                // TODO: notion-client mushes all of the text of certain BlockTypes (NumberedListItem, BulletListItem, Toggle, ToDo,
                // maybe some others) into a single Vec<Option<String>>, which is not great. When there's a need we should go back here
                // and do our own, more markdown-friendly way of extractin text for the different BlockTypes
                .plain_text()
                .into_iter()
                .map(Option::unwrap_or_default)
                .collect::<Vec<String>>()
                .join(" "), // TODO: a space " " separator is not always appropriate, but works for now. Find a better way to join the text
            creation_date: notion_block.created_time.unwrap_or_default(),
            update_date: notion_block.last_edited_time.unwrap_or_default(),
            parent: notion_block.parent,
            has_children: notion_block.has_children.unwrap_or_default(),
        }
    }

    #[must_use]
    pub fn to_markdown(&self) -> String {
        match &self.block_type {
            BlockType::Heading1 { heading_1: _ } => format!("# {}", self.text),
            BlockType::Heading2 { heading_2: _ } => format!("## {}", self.text),
            BlockType::Heading3 { heading_3: _ } => format!("### {}", self.text),
            BlockType::BulletedListItem {
                bulleted_list_item: _,
            } => format!("- {}", self.text),
            BlockType::NumberedListItem {
                numbered_list_item: _,
            } => format!("1. {}", self.text),
            BlockType::ToDo { to_do: _ } => format!("- [ ] {}", self.text),
            BlockType::Toggle { toggle: _ } => format!("> {}", self.text),
            _ => format!("{}", self.text),
        }
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

/// A Page is a container for Blocks, and its structure is heavily borrowed
/// from the Notion API's [Page object](https://developers.notion.com/reference/page).
///
/// The Page struct exists because the data sources (such as Notion) that we ingest
/// from all have the concept of a collection of Blocks, and provide APIs for fetching
/// recently edited collections of Blocks. We call those Pages.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Page {
    pub id: PageID,
    pub title: String,
    pub url: String,
    pub creation_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
    pub child_blocks: Vec<Block>,
}

#[cfg(test)]
mod tests {
    use notion_client::objects::{
        block::{BulletedListItemValue, TextColor},
        rich_text::{RichText, Text},
    };

    use super::*;

    #[test]
    fn test_block_to_markdown() {
        let blocks = vec![
            Block {
                id: BlockID("1".to_string()),
                block_type: BlockType::Heading1 {
                    heading_1: Default::default(),
                },
                text: "Heading 1".to_string(),
                creation_date: Utc::now(),
                update_date: Utc::now(),
                parent: None,
                has_children: false,
                page_id: PageID("7b1b3b0c-14cb-45a6-a4b6-d2b48faecccb".to_string()),
            },
            Block {
                id: BlockID("2".to_string()),
                block_type: BlockType::Heading2 {
                    heading_2: Default::default(),
                },
                text: "Heading 2".to_string(),
                creation_date: Utc::now(),
                update_date: Utc::now(),
                parent: None,
                has_children: false,
                page_id: PageID("7b1b3b0c-14cb-45a6-a4b6-d2b48faecccb".to_string()),
            },
            Block {
                id: BlockID("3".to_string()),
                block_type: BlockType::BulletedListItem {
                    bulleted_list_item: BulletedListItemValue {
                        rich_text: vec![RichText::Text {
                            plain_text: Some("Bullet point".to_string()),
                            href: None,
                            annotations: None,
                            text: Text {
                                content: "Bullet point".to_string(),
                                link: None,
                            },
                        }],
                        color: TextColor::Default,
                        children: None,
                    },
                },
                text: "Bullet point".to_string(),
                creation_date: Utc::now(),
                update_date: Utc::now(),
                parent: None,
                has_children: false,
                page_id: PageID("7b1b3b0c-14cb-45a6-a4b6-d2b48faecccb".to_string()),
            },
            Block {
                id: BlockID("4".to_string()),
                block_type: BlockType::Paragraph {
                    paragraph: Default::default(),
                },
                text: "Normal text".to_string(),
                creation_date: Utc::now(),
                update_date: Utc::now(),
                parent: None,
                has_children: false,
                page_id: PageID("7b1b3b0c-14cb-45a6-a4b6-d2b48faecccb".to_string()),
            },
        ];

        let expected_markdown = "# Heading 1\n## Heading 2\n- Bullet point\nNormal text";
        let result_markdown = blocks
            .iter()
            .map(|block| block.to_markdown())
            .collect::<Vec<String>>()
            .join("\n");

        assert_eq!(result_markdown, expected_markdown);
    }
}
