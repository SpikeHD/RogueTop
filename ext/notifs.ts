import { waitForElement } from './utils'

const notifSectionStyle = `
.notif-section {
  position: fixed;
  z-index: 99;
  bottom: 0;
  right: 0;

  display:  flex;
  flex-direction: column;
  align-items: flex-end;

  padding: 10px;

  background: none;
  color: white;

  font-family: sans-serif;
  font-size: 14px;

  height: 100vh;
  width: 30vw;

  pointer-events: none;
}

.notif {
  display: flex;
  justify-content: center;
  align-items: center;

  margin-bottom: 10px;
  width: 80%;
  height: 80px;

  background: #2f2f2f;
  color: #fff;

  border-radius: 10px;
  
  text-align: center;
  font-size: 1em;
  font-weight: normal;

  margin: 1em;
}
`

export async function createNotifSection() {
  const notifSection = document.createElement('div')
  const style = document.createElement('style')
  notifSection.classList.add('notif-section')

  style.innerText = notifSectionStyle

  await waitForElement('body')

  document.body.appendChild(notifSection)
  document.head.appendChild(style)
}

export async function showNotif(message: string, expire: number = 5000) {
  const notifSection = document.querySelector('.notif-section')

  if (notifSection === null) return

  const notif = document.createElement('div')
  notif.classList.add('notif')
  notif.innerText = message

  notifSection.appendChild(notif)

  setTimeout(() => {
    notif.remove()
  }, expire)
}
