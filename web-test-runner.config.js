import debug from "debug"
import { playwrightLauncher } from '@web/test-runner-playwright'
import { esbuildPlugin } from "@web/dev-server-esbuild"

const chromium = playwrightLauncher({ product: 'chromium' })
const firefox = playwrightLauncher({ product: 'firefox' })

const hostname = "localhost"
const port = 32117

const esbuild = esbuildPlugin({
  ts: true,
  tsconfig: 'tsconfig.json',
})

export default {
  watch: true,
//  debug: true,
  hostname,
  port,
  rootDir: "root",
//  middleware: [ remap ],
  plugins: [ esbuild ],
//  http2: true,
  playwright: true,
  browsers: [ chromium, firefox ],
  nodeResolve: true,
//  sslKey: "ssl/localhost.key",
//  sslCert: "ssl/localhost.crt",
}


// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+

// Copyright 2023 Daniel Ly; SPDX-License-Identifier: ISC+
