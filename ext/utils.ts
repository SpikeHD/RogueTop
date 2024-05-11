export async function waitForElement(selector: string, wait: number = 5000) {
  const per = 100
  let totalTime = 0

  return new Promise((resolve, reject) => {
    const interval = setInterval(() => {
      const element = document.querySelector(selector)

      if (element !== null) {
        clearInterval(interval)
        resolve(element)
      }

      totalTime += per

      if (totalTime >= wait) {
        clearInterval(interval)
        reject(new Error(`Element ${selector} not found after ${wait}ms`))
      }
    }, per)
  })
}
