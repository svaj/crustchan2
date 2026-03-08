import './App.css'

import { MainLayout, Container, Grid, Card, CardBody } from './components';

export function HomePage() {
  return (
    <MainLayout title="Home">
      <Container maxWidth="lg">
        <Grid columns={{ mobile: 1, tablet: 2, desktop: 3 }}>
          <Card>
            <CardBody>Card 1</CardBody>
          </Card>
          <Card>
            <CardBody>Card 2</CardBody>
          </Card>
          <Card>
            <CardBody>Card 3</CardBody>
          </Card>
        </Grid>
      </Container>
    </MainLayout>
  );
}

function App() {

  return HomePage();
}

export default App
