import { useEffect, useState } from 'react'

type Status = 'checking' | 'connected' | 'unavailable'

async function checkHealth(signal: AbortSignal): Promise<boolean> {
  const response = await fetch('/api/v1/health', { signal })
  if (response.status !== 200) return false
  const data: unknown = await response.json()
  return typeof data === 'object' && data !== null && 'status' in data && data.status === 'ok'
}

export default function App() {
  const [status, setStatus] = useState<Status>('checking')
  const [attempt, setAttempt] = useState(0)

  useEffect(() => {
    const controller = new AbortController()
    checkHealth(controller.signal)
      .then((healthy) => { if (!controller.signal.aborted) setStatus(healthy ? 'connected' : 'unavailable') })
      .catch(() => { if (!controller.signal.aborted) setStatus('unavailable') })
    return () => controller.abort()
  }, [attempt])

  function retry() {
    setStatus('checking')
    setAttempt((previous) => previous + 1)
  }

  return (
    <main>
      <h1>Palaver</h1>
      <section aria-labelledby="backend-heading">
        <h2 id="backend-heading">Backend status</h2>
        <p role="status" aria-live="polite">
          {status === 'checking' ? 'Checking connection…' : status === 'connected' ? 'Connected' : 'Unavailable'}
        </p>
        {status === 'unavailable' && <><p>Start the backend, then try again.</p><button onClick={retry}>Retry</button></>}
      </section>
    </main>
  )
}
