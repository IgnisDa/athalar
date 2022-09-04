import chalk from 'chalk';
import { SourceFile, VariableDeclarationKind } from 'ts-morph';

import { AthalarJsBinding } from '..';

const DELIMITER = chalk.redBright('"');

export const logText = (preText: string, msg: string) => {
  console.log(
    `${chalk.green('--->')} ${chalk.blue(
      preText
    )}: ${DELIMITER}${chalk.yellow.underline(msg)}${DELIMITER}`
  );
};

export const addClassValidatorBindingsToProject = async (
  sourceFile: SourceFile,
  binding: AthalarJsBinding,
  allImports: string[]
) => {
  const importSpecifier = 'class-validator';
  sourceFile.addVariableStatement({
    declarationKind: VariableDeclarationKind.Const,
    declarations: [
      {
        name: 'CONFIGURATION_VARIABLES',
        initializer:
          '[' + binding.atoms.map((a) => `"${a.name}"`).join(', ') + ']',
      },
    ],
    docs: [
      'An array containing all the configuration variables this binding contains',
    ],
  });
  sourceFile.addImportDeclaration({
    namedImports: allImports,
    moduleSpecifier: importSpecifier,
  });
  // create class
  const sourceClass = sourceFile.addClass({
    name: binding.details.className,
    isExported: true,
    docs: [
      `This class contains all the properties decorated with decorators from '${importSpecifier}'`,
    ],
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
};
