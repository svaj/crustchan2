# Layout Components

This directory contains reusable responsive layout components for the Crustchan frontend.

## Components

### Header
The main application header with navigation and optional menu toggle for mobile.

```tsx
import { Header } from './components';

<Header 
  title="Crustchan"
  onMenuToggle={() => setSidebarOpen(!sidebarOpen)}
/>
```

### Footer
Application footer with links and copyright information.

```tsx
import { Footer } from './components';

<Footer />
```

### Sidebar
Responsive sidebar navigation that collapses on mobile devices.

```tsx
import { Sidebar } from './components';

<Sidebar 
  isOpen={sidebarOpen}
  onClose={() => setSidebarOpen(false)}
/>
```

### MainLayout
Complete layout wrapper combining Header, Sidebar, and Footer.

```tsx
import { MainLayout } from './components';

<MainLayout title="My Page" showSidebar={true}>
  <p>Page content goes here</p>
</MainLayout>
```

### Container
Constrains content to a max-width with responsive padding.

```tsx
import { Container } from './components';

<Container maxWidth="lg">
  <h1>Centered Content</h1>
</Container>
```

**maxWidth options:** `sm` (640px), `md` (768px), `lg` (1024px), `xl` (1280px), `full`

### Grid
Responsive grid layout system.

```tsx
import { Grid } from './components';

// Fixed column count
<Grid columns={3} gap="md">
  <div>Item 1</div>
  <div>Item 2</div>
  <div>Item 3</div>
</Grid>

// Responsive columns
<Grid 
  columns={{ mobile: 1, tablet: 2, desktop: 3 }} 
  gap="lg"
>
  {/* Items */}
</Grid>
```

**gap options:** `sm` (0.75rem), `md` (1.5rem), `lg` (2rem)

### Card
Styled card container with optional header, body, and footer sections.

```tsx
import { Card, CardHeader, CardBody, CardFooter } from './components';

<Card>
  <CardHeader>Title</CardHeader>
  <CardBody>Content here</CardBody>
  <CardFooter>Footer info</CardFooter>
</Card>
```

## Responsive Breakpoints

- **Mobile**: < 768px
- **Tablet**: 768px - 1023px
- **Desktop**: ≥ 1024px

All components are fully responsive and automatically adapt to different screen sizes.

## Usage Example

```tsx
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
```
