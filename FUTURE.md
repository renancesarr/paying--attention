# Future Work

Ideas intentionally kept outside the MVP.

## Stronger GNOME Integration

If a GTK4/libadwaita fullscreen window is not disruptive enough on GNOME Wayland, explore a GNOME Shell extension for stronger topmost, focus, and lock-like behavior.

## Browser Activity Capture

Capture browser pages or tabs associated with a Focus Cycle so the user can later understand which links supported the Declared Task and which links caused Attention Drift.

Possible paths:

- Browser extension that records open tabs during a Focus Cycle.
- Browser history import scoped to the Focus Cycle time window.
- Manual attach/save flow for pages the user wants to preserve.

## AI-Assisted Review

Use AI only as post-processing after a Focus Cycle or later review, never as a real-time blocker for the MVP.

Possible roles:

- Summarize pages visited during a task.
- Classify whether links supported or distracted from the Declared Task.
- Generate learning notes for completed and incomplete tasks.
- Help turn unfinished curiosity trails into future Declared Tasks.

## Derived Task Lineage

When the Continuation limit is reached, a future version should store explicit lineage between the old Declared Task and the new derived task, such as `derived_from_task_id`.

## Encrypted Local Data

The MVP stores local data in plaintext under the user's profile. A future version should encrypt local task, justification, and event history at rest.

## Meeting Awareness

Improve Meeting Mode by detecting whether a selected meeting app is actually open and likely in a call.

Possible approach:

- User chooses one or more meeting apps.
- App observes whether the meeting app process is still running.
- Future version may inspect whether the app appears to be in an active call.
- If Meeting Mode is active without evidence of a call, alert every 10 minutes.

## Meeting Reflection

After Meeting Mode ends, support a structured Meeting Reflection:

- Was the meeting useful?
- What was understood?
- What decisions or next actions came out of it?
- Should the next Declared Task follow from the meeting?
