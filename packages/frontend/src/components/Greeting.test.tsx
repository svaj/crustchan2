import { render, screen } from '@testing-library/react'
import { describe, expect, it } from 'vitest'
import { Greeting } from './Greeting'

describe('Greeting', () => {
  it('renders a greeting message', () => {
    render(<Greeting name="World" />)
    expect(screen.getByRole('heading', { name: /hello, world!/i })).toBeInTheDocument()
  })
})
