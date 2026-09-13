---
name: ai-gitflow
description: Run autonomous repository work through the Paying Attention Gitflow branches, reviews, and pull requests. Use for branch creation, commits, rebases, reviews, or pull requests in this repository.
---

# AI Gitflow

Use this workflow for Paying Attention. It separates autonomous implementation from human integration.

## Branches

- `main`: release history. Accepts reviewed pull requests from `develop` only.
- `develop`: human-reviewed integration history. Accepts reviewed pull requests from `develop-with-ai` only.
- `develop-with-ai`: autonomous integration history. Accepts reviewed pull requests from short-lived work branches only.

Start every autonomous task from the latest `develop-with-ai`. Do not commit work directly on `main`, `develop`, or `develop-with-ai`.

## Work Branches

Use one of these branch names:

- `feature/<issue>-<slug>` for a new user-visible capability.
- `fix/<issue>-<slug>` for a defect correction.
- `refactor/<issue>-<slug>` for behavior-preserving restructuring.
- `test/<issue>-<slug>` for test-only changes.
- `docs/<issue>-<slug>` for documentation-only changes.
- `chore/<issue>-<slug>` for tooling or repository maintenance.

Keep a work branch scoped to one coherent issue or concern. Stage only the files belonging to that change and write focused imperative commit messages.

## Integration Flow

1. Update local knowledge of `origin/develop-with-ai`.
2. Create a work branch from that branch.
3. Implement and verify the task. Run the formatter, relevant tests, and any required manual checks.
4. Rebase the work branch onto the current `develop-with-ai`; resolve conflicts before review.
5. Run code review for the branch. Address actionable findings and re-run verification.
6. Open a pull request from the work branch to `develop-with-ai`.
7. After that work is integrated, run the automatic code review for the aggregate `develop-with-ai` change and open a pull request from `develop-with-ai` to `develop`.
8. A human reviews and tests the `develop` pull request. Only after human approval may `develop` be proposed to `main` in a separate pull request.

## Guardrails

- Never push directly to `main` or `develop`.
- Do not force-push shared branches. A rebased short-lived work branch may use `--force-with-lease` only when its history is owned by the current task.
- Do not create a pull request until its target is current, the branch has been reviewed, and required checks are green.
- Do not close an issue merely because code exists locally; close it after the relevant pull request is merged or the user explicitly directs otherwise.
- Delete a short-lived branch only after its pull request is merged and it is no longer needed for review or recovery.
- Branch protection is configured in GitHub, outside this skill. Do not weaken it.
