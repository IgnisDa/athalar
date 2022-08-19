#!/usr/bin/env node
import { command, optional, positional, run, subcommands } from 'cmd-ts';
import { ExistingPath } from 'cmd-ts/batteries/fs';
import { uniq } from 'lodash';
import { Project } from 'ts-morph';

import { AthalarJs, AthalarJsBindingType } from '..';
import { logText } from '../js-src';

const GENERATE_SUBCOMMAND = 'generate';

const generateCmd = command({
  name: GENERATE_SUBCOMMAND,
  args: {
    path: positional({
      type: optional(ExistingPath),
      description:
        'The path where the athalar project is present, defaults to $PWD',
    }),
  },
  handler: async ({ path }) => {
    if (!path) path = process.cwd();
    const ath = AthalarJs.fromPath(path);
    // const report = ath.getValidationReports();
    for (const bindingType of [
      { import: 'class-validator', type: AthalarJsBindingType.ClassValidator },
    ]) {
      const bindings = ath.getInformation(bindingType.type);
      const project = new Project();
      for (const binding of bindings) {
        logText(`Processing binding for`, binding.output);
        const sourceFile = project.createSourceFile(binding.output, undefined, {
          overwrite: true,
        });
        sourceFile.addImportDeclaration({
          namedImports: uniq(
            bindings.flatMap((b) => b.atoms.flatMap((a) => a.validators))
          ),
          moduleSpecifier: bindingType.import,
        });
        // create class
        const sourceClass = sourceFile.addClass({
          name: binding.details.className,
          isExported: true,
        });
        for (const atom of binding.atoms) {
          // add properties to the class
          sourceClass.addProperty({
            name: atom.name,
            type: atom.kind,
            decorators: atom.validators.map((v) => ({
              name: v,
              arguments: [],
            })),
            docs: atom.description ? [atom.description] : [],
          });
        }
        for (const file of project.getSourceFiles()) {
          // console.log(file.getFullText());
          await file.save();
        }
      }
    }
  },
});

const mainCmd = subcommands({
  name: 'athalar',
  description: 'Generate bindings for a particular athalar project',
  cmds: { [GENERATE_SUBCOMMAND]: generateCmd },
});

run(mainCmd, process.argv.slice(2));
