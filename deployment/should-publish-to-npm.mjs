import caporal from '@caporal/core';
import axios from 'axios';
import { execa } from 'execa';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import semver from 'semver';

const workspaceOutputPath = join('/tmp', 'workspace.json');

const main = async () => {
  await execa('pnpm', ['nx', 'graph', `--file=${workspaceOutputPath}`]);
  const nodes = JSON.parse(
    readFileSync(workspaceOutputPath, { encoding: 'utf-8' })
  ).graph.nodes;
  const deployableProjects = Object.fromEntries(
    Object.entries(nodes).filter(([_key, value]) =>
      value.data.tags.includes('publish-npm')
    )
  );
  caporal.program
    .name('Should publish project to NPM')
    .description(
      `
Calculates whether the JS library needs to be published.
It returns a non-zero exit code if it needs to be published else zero.
`
    )
    .action(async ({ logger }) => {
      const packageName = `@ignisda/athalar`;
      logger.info(`Testing whether '${packageName}' should be released`);
      const packageJson = JSON.parse(
        readFileSync(`apps/athalar-js/package.json`, { encoding: 'utf-8' })
      );
      const versionOnLocal = packageJson.version;
      logger.info(`New version ready to be published: '${versionOnLocal}'`);
      try {
        const { data } = await axios.get(
          `https://registry.npmjs.org/${packageName}`,
          { headers: { Accept: 'application/vnd.npm.install-v1+json' } }
        );
        const versionOnNpm = data['dist-tags'].latest;
        logger.info(`Latest version in NPM is: '${versionOnNpm}'`);
        if (semver.gt(versionOnLocal, versionOnNpm)) {
          logger.warn(
            `Since local::'${versionOnLocal}' > npm::'${versionOnNpm}', this package needs to be published`
          );
          process.exit(1);
        } else {
          logger.warn(
            `Since local::'${versionOnLocal}' === npm::'${versionOnNpm}', this package does not need to be published`
          );
          return;
        }
      } catch {
        logger.warn(
          `This project does not exist on NPM, so it needs to be published`
        );
        process.exit(1);
      }
    });
  caporal.program.run();
};

main();
