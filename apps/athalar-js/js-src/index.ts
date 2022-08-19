import chalk from 'chalk';

const DELIMITER = chalk.redBright('"');

export const logText = (preText: string, msg: string) => {
  console.log(
    `${chalk.green('--->')} ${chalk.blue(
      preText
    )}: ${DELIMITER}${chalk.yellow.underline(msg)}${DELIMITER}`
  );
};
