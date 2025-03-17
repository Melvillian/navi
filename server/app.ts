import bootstrap from './lib/bootstrap';

const app = bootstrap();

const appPort = process.env.PORT || 3000;

app.listen(appPort, () => {
  console.log(`Server started on port ${appPort}`);
});
