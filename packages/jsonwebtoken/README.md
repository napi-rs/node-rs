# `@node-rs/jsonwebtoken`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/jsonwebtoken.svg?sanitize=true)

🚀 Fastest jsonwebtoken in Node.js

## Usage

See [tests](__tests__/jsonwebtoken.spec.ts) and [types](index.d.ts)

## Bench

```
                  ,MMMM.           Host        -  xxxxxxxxxxxxxxxxxxxxxxx
                .MMMMMM            Machine     -  Mac15,9
                MMMMM,             Kernel      -  24.0.0
      .;MMMMM:' MMMMMMMMMM;.       OS          -  macOS 15.0.1 Sequoia
    MMMMMMMMMMMMNWMMMMMMMMMMM:     DE          -  Aqua
  .MMMMMMMMMMMMMMMMMMMMMMMMWM.     WM          -  Quartz Compositor
  MMMMMMMMMMMMMMMMMMMMMMMMM.       Packages    -  194 (Homebrew), 32 (cargo)
 ;MMMMMMMMMMMMMMMMMMMMMMMM:        Shell       -  zsh
 :MMMMMMMMMMMMMMMMMMMMMMMM:        Terminal    -  warpterminal (Version v0.2024.10.23.14.49.stable_00)
 .MMMMMMMMMMMMMMMMMMMMMMMMM.       Resolution  -  5120x2880@160fps (as 2560x1440)
  MMMMMMMMMMMMMMMMMMMMMMMMMMM.                    2992x1934@120fps (as 1496x967)
   .MMMMMMMMMMMMMMMMMMMMMMMMMM.                   2232x1512@60fps (as 1116x756)
     MMMMMMMMMMMMMMMMMMMMMMMM      Uptime      -  1d 2h 32m
      ;MMMMMMMMMMMMMMMMMMMM.       CPU         -  Apple M3 Max (16)
        .MMMM,.    .MMMM,.         CPU Load    -  16%
                                   Memory      -  50.1 GB / 134.2 GB
                                   Battery     -  78% & Discharging
                                   Disk Space  -  624.0 GB / 994.7 GB
```

```text
❯ yarn workspace @node-rs/jsonwebtoken bench
Sign token
┌─────────┬──────────────────────────────┬───────────┬────────────────────┬──────────┬─────────┐
│ (index) │ Task Name                    │ ops/sec   │ Average Time (ns)  │ Margin   │ Samples │
├─────────┼──────────────────────────────┼───────────┼────────────────────┼──────────┼─────────┤
│ 0       │ '@node-rs/jsonwebtoken'      │ '91,244'  │ 10959.520789952572 │ '±0.26%' │ 45623   │
│ 1       │ '@node-rs/jsonwebtoken sync' │ '327,344' │ 3054.88582722852   │ '±0.27%' │ 163673  │
│ 2       │ 'jsonwebtoken'               │ '59,972'  │ 16674.345716476044 │ '±1.26%' │ 29987   │
│ 3       │ 'jsonwebtoken sync'          │ '70,192'  │ 14246.519901985555 │ '±1.50%' │ 35097   │
└─────────┴──────────────────────────────┴───────────┴────────────────────┴──────────┴─────────┘
Verify token
┌─────────┬──────────────────────────────┬───────────┬────────────────────┬──────────┬─────────┐
│ (index) │ Task Name                    │ ops/sec   │ Average Time (ns)  │ Margin   │ Samples │
├─────────┼──────────────────────────────┼───────────┼────────────────────┼──────────┼─────────┤
│ 0       │ '@node-rs/jsonwebtoken'      │ '99,215'  │ 10079.106313498452 │ '±0.20%' │ 49608   │
│ 1       │ '@node-rs/jsonwebtoken sync' │ '299,184' │ 3342.421242972733  │ '±0.23%' │ 149593  │
│ 2       │ 'jsonwebtoken'               │ '53,938'  │ 18539.495365223007 │ '±1.51%' │ 26970   │
│ 3       │ 'jsonwebtoken sync'          │ '58,633'  │ 17055.03492853964  │ '±1.27%' │ 29317   │
└─────────┴──────────────────────────────┴───────────┴────────────────────┴──────────┴─────────┘
```
