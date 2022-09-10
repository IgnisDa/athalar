#!/usr/bin/env node
import { command, optional, positional, run, subcommands } from 'cmd-ts';
import { ExistingPath } from 'cmd-ts/batteries/fs';
import { uniq } from 'lodash';
import { Project } from 'ts-morph';

import { AthalarJs, AthalarJsBindingType } from '..';
import { addClassValidatorBindingsToProject, BINARY, logText } from '../js-src';

const GENERATE_SUBCOMMAND = 'generate';
const LEADING_COMMENT = `/*
 * -----------------------------------------------------
 * THIS FILE WAS AUTOMATICALLY GENERATED (DO NOT MODIFY)
 * -----------------------------------------------------
 */`;

const generateCommand = command({
  name: GENERATE_SUBCOMMAND,
  args: {
    path: positional({
      type: optional(ExistingPath),
      description: `The path where the ${BINARY} project is present, defaults to "$PWD"`,
    }),
  },
  handler: async ({ path }) => {
    if (!path) {
      path = process.cwd();
      logText(`No path provided, using`, path);
    }
    try {
      const athalarProject = AthalarJs.fromPath(path);
      // const report = ath.getValidationReports();
      const project = new Project();
      const bindings = athalarProject.getInformation();
      for (const binding of bindings) {
        const sourceFile = project.createSourceFile(binding.output, undefined, {
          overwrite: true,
        });
        sourceFile.addStatements(LEADING_COMMENT);
        if (binding.variety === AthalarJsBindingType.ClassValidator) {
          logText(`Processing binding for`, binding.output);
          const allImports = uniq(
            bindings.flatMap((b) => b.atoms.flatMap((a) => a.validators))
          );
          await addClassValidatorBindingsToProject(
            sourceFile,
            binding,
            allImports
          );
        }
      }
      for (const file of project.getSourceFiles()) {
        file.formatText();
        console.log(file.getFullText());
        await file.save();
      }
    } catch (e: any) {
      console.error(`Encountered error: "${e.message}"`);
    }
  },
});

const mainCommand = subcommands({
  name: BINARY,
  description: `Generate bindings for a particular ${BINARY} project`,
  cmds: { [GENERATE_SUBCOMMAND]: generateCommand },
});

run(mainCommand, process.argv.slice(2));
