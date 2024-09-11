import fs from 'node:fs/promises'
import { fileURLToPath } from 'node:url'

export const ABI = JSON.parse(await fs.readFile(
  fileURLToPath(new URL('./Abi.json', import.meta.url))
))

export const ADDRESS = '0xffac3d46e0adfd7806c454b23c3c74d95e09bb02'
