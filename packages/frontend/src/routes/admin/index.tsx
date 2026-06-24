import type { JSX } from 'react'
import type { TuonoRouteProps } from 'tuono'

export interface AdminIndexData {
  title: string
}

export default function AdminIndexPage({
  isLoading,
  data,
}: TuonoRouteProps<AdminIndexData>): JSX.Element {
  if (isLoading || !data) {
    return <p>Loading…</p>
  }

  return (
    <div>
      <h1 style={headingStyle}>{data.title}</h1>
      <div style={gridStyle}>
        <StatCard label="Users" value={1284} />
        <StatCard label="Bans" value={37} />
        <StatCard label="Posts" value={89201} />
      </div>
    </div>
  )
}

function StatCard({ label, value }: { label: string; value: number }): JSX.Element {
  return (
    <div style={cardStyle}>
      <div style={cardLabelStyle}>{label}</div>
      <div style={cardValueStyle}>{value.toLocaleString()}</div>
    </div>
  )
}

const headingStyle: React.CSSProperties = {
  fontSize: '24px',
  fontWeight: 700,
  marginBottom: '20px',
  letterSpacing: '-0.5px',
  color: '#1a1a1a',
}

const gridStyle: React.CSSProperties = {
  display: 'grid',
  gridTemplateColumns: 'repeat(auto-fill, minmax(180px, 1fr))',
  gap: '16px',
}

const cardStyle: React.CSSProperties = {
  background: '#fff',
  borderRadius: '8px',
  padding: '20px',
  boxShadow: '0 1px 3px rgba(0,0,0,0.08)',
}

const cardLabelStyle: React.CSSProperties = {
  fontSize: '12px',
  fontWeight: 600,
  textTransform: 'uppercase',
  color: '#888',
  marginBottom: '8px',
}

const cardValueStyle: React.CSSProperties = {
  fontSize: '28px',
  fontWeight: 700,
  color: '#1a1a1a',
  letterSpacing: '-0.5px',
}
