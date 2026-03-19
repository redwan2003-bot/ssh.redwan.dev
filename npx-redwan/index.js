#!/usr/bin/env node

const chalk = require('chalk');

// Parse flags
const args = process.argv.slice(2);
const isContact = args.includes('--contact');

if (isContact) {
    console.log(`
${chalk.cyan.bold('Redwan Ahmmed')} | ${chalk.yellow('Contact Signal')}
${chalk.dim('------------------------------------------------------------')}

${chalk.green('[ENCRYPTED CONTACT DATA]')}
> Email:     ${chalk.white.bold('reahs302444@gmail.com')}
> LinkedIn:  ${chalk.white.bold('https://linkedin.com/in/redwanahmmed')}
> GitHub:    ${chalk.white.bold('https://github.com/redwan2003-bot')}

${chalk.dim('Signal: reahs302444@gmail.com')}
    `);
    process.exit(0);
}

console.log(`
${chalk.cyan.bold('Redwan Ahmmed')} | ${chalk.yellow('Hardware R&D & IoT Specialist')}
${chalk.dim('------------------------------------------------------------')}

${chalk.green('[SYSTEM STATUS]')}
> Role:      Embedded Systems Engineer (Titan-Core)
> Location:  Dhaka, Bangladesh
> Specialty: 4-Layer PCB Design | ESP32 Gateways | Robotics

${chalk.green('[ACTIVE PROJECTS]')}
* ${chalk.white.bold('Titan-Core:')} High-performance industrial controller.
* ${chalk.white.bold('Mars Rover:')} UIU Robotics - Navigation & Power systems.
* ${chalk.white.bold('IoT Gateway:')} Custom ESP32-based mesh network.

${chalk.green('[CONNECT]')}
- GitHub:    https://github.com/redwan2003-bot
- LinkedIn:  https://linkedin.com/in/redwanahmmed
- Portfolio: https://ssh.redwan.dev

${chalk.dim('Type "npx redwan-ahmmed-portfolio --contact" for encrypted email signal.')}
`);
