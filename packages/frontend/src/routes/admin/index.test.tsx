import { render, screen } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import AdminIndexPage from './index'

describe('AdminIndexPage', () => {
  it('renders the admin dashboard heading and stat cards', () => {
    render(<AdminIndexPage isLoading={false} data={{ title: 'Crustchan Admin' }} />)

    expect(screen.getByRole('heading', { name: /crustchan admin/i })).toBeInTheDocument()
    expect(screen.getByText('Users')).toBeInTheDocument()
    expect(screen.getByText('1,284')).toBeInTheDocument()
    expect(screen.getByText('Bans')).toBeInTheDocument()
    expect(screen.getByText('37')).toBeInTheDocument()
    expect(screen.getByText('Posts')).toBeInTheDocument()
    expect(screen.getByText('89,201')).toBeInTheDocument()
  })

  it('renders loading state when isLoading is true', () => {
    render(<AdminIndexPage isLoading={true} data={null} />)
    expect(screen.getByText(/loading…/i)).toBeInTheDocument()
  })
})
