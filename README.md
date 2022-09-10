# Athalar

Athalar is a specification for generating configuration. It enables you to write your
configuration in a single format and is especially useful when you want to share
configurations across multiple languages. It transforms your configuration into language
bindings specific to your requirements.

## Summary

- [Athalar](#athalar)
  - [Summary](#summary)
  - [Example](#example)
  - [Usage](#usage)
    - [Concepts](#concepts)
      - [generator(s)](#generators)
      - [partial(s)](#partials)
    - [Configuration](#configuration)
    - [Bindings](#bindings)
  - [License](#license)

## Example

Lets consider the following mappings:

```yaml
# backend.ath.yaml
bindings:
  - output: ../../libs/generated/src/backend.ts
    profile: !ClassValidator
      class_name: ApplicationConfig

config:
  - !IncludePartial mail
```

```yaml
# partials/mail.ath.yaml
config:
  - name: MAIL_PORT
    description: The port at which the mail server is listening at
    validators:
      - !Port
  - name: MAIL_HOST
    description: The hostname where the email server is listening at
    kind: !String
```

Calling the Athalar CLI will get the following configuration auto-magically generated:

```typescript
// some/path/generated/backend.ts
import { Allow, IsPort } from 'class-validator';

const CONFIGURATION_VARIABLES = ['MAIL_PORT', 'MAIL_HOST'];

export class ApplicationConfig {
  /** The port at which the mail server is listening at */
  @IsPort()
  MAIL_PORT: number;

  /** The hostname where the email server is listening at */
  @Allow()
  MAIL_HOST: string;
}
```

Take a look at the [config](./apps/config) directory for a more complete example.

## Usage

Athalar files are just plain yaml files which are super-charged to provide additional
functionality. It leverages YAML [tags](https://yaml.org/spec/1.2.2/#24-tags) to achieve
this.

### Concepts

An athalar project consists of the following components: generators and partials.

#### generator(s)

A generator is what ends up being translated to code. Each generator consists of at-least
one "binding" and one "config". A "binding" defines how the final code should be adapted. A
"config" specifies what all variables should be specified in the output.

The [example](#example) generator (`backend.ath.yaml`) defines one binding. It uses the
[`ClassValidator`](./apps/athalar-js/) profile, specifies where the final output should be
written, and changes the name of the class generated to `ApplicationConfig`.

#### partial(s)

A partial is what defines configuration. It should contain at-least one variable under the
"config" key.

The [example](#example) partial (`mail.ath.yaml`) defines two different configuration
variables. The `MAIL_PORT` variable requires the `Port` validator. The `kind` (which is the
final type that ends up in the generated output) is automatically determined from the
validators but can also be specified explicitly.

### Configuration

Athalar projects live under a common directory and all files have a common prefix -
`*.ath.yaml`. An `athalar.toml` file is also expected at the root of the project. This can
be used to configure the behavior of athalar.

```yaml
# this is necessary
version = "1"

# the following are the defaults
source = "src/" # the directory where all the project files are located
partials = "partials/" # the directory (relative to `source`) where the partials are kept
generators = "generators/" # the directory (relative to `source`) where the generators are kept
```

### Bindings

Once you have written down your configuration using the above rules, you can use specific
language bindings to generate the final configurations.

| Language                        | Available bindings |
| ------------------------------- | ------------------ |
| [Typescript](./apps/athalar-js) | Class Validator    |

More information about the generators can be found in their specific projects.

## License

MIT © [IgnisDa](https://github.com/ignisda)
