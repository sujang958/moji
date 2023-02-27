import "./style.css"
import App from "./App.svelte"
import { globalShortcut } from "@tauri-apps/api"
import { LogicalPosition, appWindow } from "@tauri-apps/api/window"

const SHORTCUT = "Control+Shift+A"
const POSITION_KEY = "position"

globalShortcut.isRegistered(SHORTCUT).then(async (yes) => {
  if (yes) return

  globalShortcut.register(SHORTCUT, async () => {
    if (await appWindow.isVisible()) return appWindow.hide()
    else return appWindow.show()
  })
})

appWindow.onMoved((position) => {
  localStorage.setItem(
    POSITION_KEY,
    JSON.stringify({
      x: position.payload.x,
      y: position.payload.y,
    })
  )
})

const position = JSON.parse(localStorage.getItem(POSITION_KEY) ?? "{}")

if (position && "x" in position && "y" in position)
  appWindow.setPosition(new LogicalPosition(position.x, position.y))

const app = new App({
  target: document.getElementById("app"),
})

export default app
