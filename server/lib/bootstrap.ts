import express from 'express';
import path from 'path';
import morgan from 'morgan';
import helmet from 'helmet';
import cors from 'cors';
import setupRoutes from './routes';
import dotenv from 'dotenv';

dotenv.config();

const app = express();

// Since we are not using ts-node, and we are running from the dist folder,
// the views folder is not available in the dist folder when developing.
const viewsPath =
  process.env.NODE_ENV === 'production' ? '../views' : '../../../server/views';

app
  // HTTP request logger middleware for node.js
  .use(morgan(process.env.NODE_ENV === 'production' ? 'common' : 'dev'))
  // Serve static content for the app from the "assets" directory
  .use('/a', express.static(path.join(__dirname, '../../../assets')))
  .use(
    helmet({
      // If you get stuck in CSP, try this: crossOriginEmbedderPolicy: false,
      contentSecurityPolicy: {
        directives: {
          'script-src': ["'self'", "'unsafe-inline'", 'unpkg.com'],
        },
      },
    })
  )
  .use(cors())
  .use(express.json())
  .set('views', path.join(__dirname, viewsPath))
  .set('view engine', 'ejs');

setupRoutes(app);

export default () => app;
