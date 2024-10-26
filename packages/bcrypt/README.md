# `@node-rs/bcrypt`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/bcrypt.svg?sanitize=true)

🚀 Fastest bcrypt in Node.js

## Usage

```typescript
export const DEFAULT_COST: 12

export function hashSync(password: string | Buffer, round?: number): string
export function hash(password: string | Buffer, round?: number): Promise<string>
export function verifySync(password: string | Buffer, hash: string | Buffer): boolean
export function verify(password: string | Buffer, hash: string | Buffer): Promise<boolean>
/**
 * The same with `verifySync`
 */
export function compareSync(password: string | Buffer, hash: string | Buffer): boolean
/**
 * The same with `verify`
 */
export function compare(password: string | Buffer, hash: string | Buffer): Promise<boolean>

export type Version = '2a' | '2x' | '2y' | '2b'
/**
 * @param version default '2b'
 */
export function genSaltSync(round: number, version?: Version): string
/**
 * @param version default '2b'
 */
export function genSalt(round: number, version?: Version): Promise<string>
```

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
❯ yarn workspace @node-rs/bcrypt bench
Hash benchmark
┌─────────┬───────────────────┬─────────┬───────────────────┬──────────┬─────────┐
│ (index) │ Task Name         │ ops/sec │ Average Time (ns) │ Margin   │ Samples │
├─────────┼───────────────────┼─────────┼───────────────────┼──────────┼─────────┤
│ 0       │ '@node-rs/bcrypt' │ '20'    │ 49142200.63636367 │ '±1.08%' │ 11      │
│ 1       │ 'node bcrypt'     │ '20'    │ 49259219.81818187 │ '±1.35%' │ 11      │
│ 2       │ 'bcryptjs'        │ '17'    │ 58142116.79999998 │ '±0.36%' │ 10      │
│ 3       │ 'wasm OpenBSD'    │ '17'    │ 58318899.99999994 │ '±0.23%' │ 10      │
│ 4       │ 'wasm Openwall'   │ '18'    │ 53324629.20000016 │ '±0.37%' │ 10      │
└─────────┴───────────────────┴─────────┴───────────────────┴──────────┴─────────┘
Verify benchmark
┌─────────┬───────────────────┬─────────┬────────────────────┬──────────┬─────────┐
│ (index) │ Task Name         │ ops/sec │ Average Time (ns)  │ Margin   │ Samples │
├─────────┼───────────────────┼─────────┼────────────────────┼──────────┼─────────┤
│ 0       │ '@node-rs/bcrypt' │ '5'     │ 192946879.09999993 │ '±0.52%' │ 10      │
│ 1       │ 'node bcrypt'     │ '5'     │ 199806404.2        │ '±0.36%' │ 10      │
│ 2       │ 'bcryptjs'        │ '4'     │ 231329516.79999986 │ '±0.13%' │ 10      │
└─────────┴───────────────────┴─────────┴────────────────────┴──────────┴─────────┘
GenSalt benchmark
┌─────────┬───────────────────┬─────────────┬────────────────────┬──────────┬─────────┐
│ (index) │ Task Name         │ ops/sec     │ Average Time (ns)  │ Margin   │ Samples │
├─────────┼───────────────────┼─────────────┼────────────────────┼──────────┼─────────┤
│ 0       │ '@node-rs/bcrypt' │ '4,421,897' │ 226.14727567195578 │ '±0.08%' │ 2210949 │
│ 1       │ 'node bcrypt'     │ '4,477,408' │ 223.34345972377827 │ '±0.08%' │ 2238705 │
│ 2       │ 'bcryptjs'        │ '821,556'   │ 1217.2015511950851 │ '±0.39%' │ 410779  │
│ 3       │ 'wasm OpenBSD'    │ '3,685,603' │ 271.3259889021118  │ '±0.23%' │ 1842802 │
│ 4       │ 'wasm Openwall'   │ '1,462,251' │ 683.8769076754866  │ '±2.50%' │ 731126  │
└─────────┴───────────────────┴─────────────┴────────────────────┴──────────┴─────────┘
```
