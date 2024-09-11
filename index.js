import fs from 'node:fs/promises'
import { fileURLToPath } from 'node:url'

export const ABI = JSON.parse(await fs.readFile(
  fileURLToPath(new URL('./Abi.json', import.meta.url))
))

export const ADDRESS = '0xcb3e3291a298a44224bc3bafd04957e9feed5767'
