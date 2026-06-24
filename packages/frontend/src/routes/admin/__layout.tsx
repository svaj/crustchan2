import type { JSX } from 'react'
import { Link } from 'tuono'
import type { TuonoLayoutProps } from 'tuono'

export default function AdminLayout({
  children,
}: TuonoLayoutProps): JSX.Element {
  return (
    <div style={layoutStyle}>
      <aside style={sidebarStyle}>
        <nav style={navStyle} aria-label="Admin navigation">
          <h1 style={brandStyle}>Crustchan Admin</h1>
          <ul style={navListStyle}>
            <li>
              <Link href="/admin" style={navLinkStyle}>
                Dashboard
              </Link>
            </li>
            <li>
              <Link href="/admin/users" style={navLinkStyle}>
                Users
              </Link>
            </li>
            <li>
              <Link href="/admin/bans" style={navLinkStyle}>
                Bans
              </Link>
            </li>
            <li>
              <Link href="/admin/posts" style={navLinkStyle}>
                Posts
              </Link>
            </li>
          </ul>
        </nav>
      </aside>
      <section style={mainStyle}>{children}</section>
    </div>
  )
}

const layoutStyle: React.CSSProperties = {
  position: 'fixed',
  inset: 0,
  display: 'flex',
  fontFamily:
    "'Poppins', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
  zIndex: 10,
  maxWidth: 'none',
  margin: 0,
}

const sidebarStyle: React.CSSProperties = {
  width: '220px',
  background: '#1a1a1a',
  color: '#ccc',
  padding: '16px',
  flexShrink: 0,
}

const navStyle: React.CSSProperties = {
  display: 'flex',
  flexDirection: 'column',
  gap: '12px',
}

const brandStyle: React.CSSProperties = {
  fontSize: '18px',
  fontWeight: 700,
  color: '#fff',
  marginBottom: '16px',
  letterSpacing: '-0.5px',
}

const navListStyle: React.CSSProperties = {
  listStyle: 'none',
  padding: 0,
  margin: 0,
  display: 'flex',
  flexDirection: 'column',
  gap: '8px',
}

const navLinkStyle: React.CSSProperties = {
  display: 'block',
  padding: '8px 12px',
  borderRadius: '6px',
  color: '#ccc',
  textDecoration: 'none',
  fontSize: '14px',
  fontWeight: 500,
  transition: 'background 0.15s, color 0.15s',
}

const mainStyle: React.CSSProperties = {
  flex: 1,
  padding: '24px',
  background: '#f4f4f4',
  overflow: 'auto',
}
