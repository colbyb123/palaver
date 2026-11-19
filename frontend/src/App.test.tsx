import { fireEvent, render, screen } from '@testing-library/react'
import { describe, expect, it, vi } from 'vitest'
import App from './App'

const response = (body: string, status = 200) => new Response(body, { status, headers: { 'Content-Type': 'application/json' } })

describe('backend status', () => {
  it('shows checking while the request is pending and then connected', async () => {
    let resolve!: (value: Response) => void
    const fetchMock = vi.fn(() => new Promise<Response>((done) => { resolve = done }))
    vi.stubGlobal('fetch', fetchMock)
    render(<App />)
    expect(screen.getByRole('status')).toHaveTextContent('Checking connection')
    resolve(response('{"status":"ok"}'))
    expect(await screen.findByText('Connected')).toBeInTheDocument()
    expect(fetchMock).toHaveBeenCalledWith('/api/v1/health', expect.objectContaining({ signal: expect.any(AbortSignal) }))
  })

  it.each([
    ['HTTP failure', () => Promise.resolve(response('{}', 503))],
    ['invalid JSON', () => Promise.resolve(response('not json'))],
    ['unexpected data', () => Promise.resolve(response('{"status":"error"}'))],
    ['network failure', () => Promise.reject(new Error('offline'))],
  ])('shows unavailable for %s', async (_, fetchResult) => {
    vi.stubGlobal('fetch', vi.fn(fetchResult))
    render(<App />)
    expect(await screen.findByText('Unavailable')).toBeInTheDocument()
  })

  it('retries after a failure', async () => {
    vi.stubGlobal('fetch', vi.fn().mockRejectedValueOnce(new Error('offline')).mockResolvedValueOnce(response('{"status":"ok"}')))
    render(<App />)
    fireEvent.click(await screen.findByRole('button', { name: 'Retry' }))
    expect(await screen.findByText('Connected')).toBeInTheDocument()
  })
})
