import { Express, Request, Response } from 'express';

export default (app: Express) =>
  app

    .get('/', async (req: Request, res: Response) => {
      res.render('index', {
        currentTime: new Date().toISOString(),
      });
    })

    .get('/api/server-time', async (req: Request, res: Response) => {
      res.render('partials/serverTime', {
        currentTime: new Date().toISOString(),
      });
    });
