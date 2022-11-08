# Contributing

Thanks for taking the time to contribute to this project! Depending on what you'd like to help out with, there are a few rules and guidelines to follow.

Before getting started, make sure to read the [Code of Conduct](./CODE_OF_CONDUCT.md) - keeping this space as civil as possible is crucial for collaboration.

Here are a few ways you can help:
- [Report a bug/issue](./CONTRIBUTING.md#report-a-bugissue)
- [Suggest a feature](./CONTRIBUTING.md#Suggest-a-feature)
- [Contribute code](./CONTRIBUTING.md#Contribute-code)
- [Write documentation](./CONTRIBUTING.md#writing-documentation)
- [Something else?](./CONTRIBUTING.md#something-else)

## Report a bug/issue
Found something wrong with the app?

- Check if someone already reported it by searching the [Issues](https://github.com/arqalite/rummy-nights/issues) page.
- If nothing comes up, create a new issue. 
  - Include as much information as you can, every little bit helps.
  - Try to follow the template as much as possible, but don't be afraid to add new things to the issue. 
  - If something is not applicable, write "N/A" or provide context as to why it's not applicable. Don't leave it blank please.
- If you have Rust/Dioxus experience but you're not able/willing to [try and fix the issue](./CONTRIBUTING.md#Contribute-code), try to come up with a possible solution that we can try and implement. 

## Suggest a feature
Do you have an idea that could make the app more useful/fun/easier to use?

- Check if someone already suggested your idea by searching the [Issues](https://github.com/arqalite/rummy-nights/issues) page.
- If nothing comes up, create a new issue. 
  - Provide context; let us know the reasoning behind your idea, the use case, and how relevant it is to the userbase.
  - If you have the time, offer as many details as you can. Share implementation ideas, how it should be designed, how it should be used, pros and cons, anything that comes to mind.
- If you have Rust/Dioxus experience but you're not able/willing to [develop this feature yourself](./CONTRIBUTING.md#Contribute-code), provide tips or theoretical implementation details (useful crates, standard library types/functions, etc.)

## Contribute code
Help us make the app better!

First, let's determine what kind of contribution you want to make - patching a bug, adding a new feature, performance optimizations or just increasing code quality/refactoring?

#### Bug patches and performance optimizations
- If you want to fix a bug or change some code to be more efficient, make sure an [issue](https://github.com/arqalite/rummy-nights/issues) for it exists. If not, create one by following the template.
- Fork the repo, write your patch, and test thoroughly!
- [Create and submit your pull request](./CONTRIBUTING.md#Pull-requests).

#### Adding new features
- If an issue for it exists and is being tracked for a future release (has a milestone on it), start working on it directly. As with bug fixes, fork the repo, write code, and test thoroughly.
- If an issue doesn't exist for the feature you want to add - don't start writing code yet! 
  - Please first create an issue explaining the feature and what needs to be implemented for it to work. 
  - We'll discuss the feature together and determine if it should be added to the app or not, and gather feedback on it. 
  - Once the issue has a milestone added to it, you can start working on it. 
  - Keep the issue updated with information, technical details and thoughts you feel are relevant.
  - Once the feature is in good shape, [create and submit your pull request](./CONTRIBUTING.md#Pull-requests).

#### Refactoring
This one is more complicated than it seems. 

We hate bad code as much as the next person, but oftentimes the risks outweigh the benefits. We reserve the right to approve/reject refactoring PRs for any reason, even if the code seems spotless. Here's an interesting read on why refactoring PRs aren't a good idea: https://mdipenta.github.io/files/tosem-rejected-refact-2022.pdf

In short, refactoring big chunks of code can introduce bugs, make code less efficient or less readable (weird, right? The point of refactoring is to not do any of that, but it's quite hard to avoid in practice). On the other hand, refactoring only small bits and pieces of code and manually validate them to be correct solves the bug problem, but takes much more time and resources, and clutters the repo with tiny commits with no actual benefit to the end user experience (which is what matters most).

If you still want to refactor code - raise an issue first to discuss with the team.

#### Commits and pull requests
- Once your code is ready, please lint it with `cargo clippy`. Implement any suggestions, and run it again. If nothing shows up, then you can format the code with `cargo fmt`. This keeps the code to a certain standard and it only takes a couple of moments.
- Before submitting the pull request, squash your commits and include the issue number in the commit message. Squashing prevents git log clutter, and including the issue number helps for tracking changes as needed.
- Our commits follow a more relaxed version of the [Conventional Commits specification](https://www.conventionalcommits.org/en/v1.0.0/). So far, we make use of the following commit types:
  - `fix` - for bug fixes, patches that resolve a problem with the app
  - `feat` - new features, stuff that is user-facing and interactable in the app
  - `perf` - performance tweaks, optimizing code and reducing the amount of resources used
  - `refactor` - rewriting code, formatting, increasing code quality without changing app functionality or performance
  - `ci` - changes to GitHub actions/workflows, Vercel stuff, or to any automated tasks
  - `doc` - changes to documentation, wikis, license and other texts (like this document!)
  
  Example commits: `feat: add tile bonus functionality`, `fix: read storage regardless of game status`, `refactor: moved web assets to their own folder`
  You can use other types if needed, but try to stick to these ones as much as possible.
- Pull requests will be reviewed and approved/denied at the team's discretion, but in 99% of cases we'll provide context on why it was denied and/or try to work with the author to get it approved.

## Writing documentation
This is currently our weakest point - so any help is appreciated.

At the moment we are open to any PRs regarding documentation, be it code comments, wiki pages, in-app help screens or other texts. Feel free to submit anything you think will be useful.

## Something else?
If you want to help us out in a way that's not detailed here - reach out to us and we'll point you in the right direction.
