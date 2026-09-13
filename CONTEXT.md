# Paying Attention

Paying Attention is a Linux desktop attention-management app that interrupts passive computer use and requires the user to consciously state what they are doing next.

## Language

**Attention Block**:
A blocking full-screen moment where the user must respond before returning to normal desktop use.
_Avoid_: popup, reminder, notification

**Check-in**:
The first Attention Block after login, where the user records context and declares the current task.
_Avoid_: onboarding, setup

**Focus Cycle**:
A timed work interval after an Attention Block has been released.
_Avoid_: pomodoro, sprint

**Review**:
An Attention Block after a Focus Cycle where the user evaluates the previous task and declares the next task.
_Avoid_: retrospective, report

**Declared Task**:
The task text the user explicitly commits to during a Check-in or Review.
_Avoid_: todo, goal, reminder

**Continuation**:
Choosing to keep working on the same Declared Task for another Focus Cycle.
_Avoid_: retry, snooze, postpone

**Nagging Mode**:
The intentionally punitive attention pressure activated when the user leaves an Attention Block idle or stops interacting during a Focus Cycle.
_Avoid_: gentle reminder, passive notification

**Attention Drift**:
A moment where the user has left the declared focus path, either into offline distraction or unbounded curiosity such as link-hopping.
_Avoid_: failure, laziness

**Completion Justification**:
The user's explanation of what happened when a Declared Task was not completed during a Focus Cycle.
_Avoid_: excuse, confession

**Meeting Mode**:
A temporary user-declared pause for calls or meetings where the attention pressure would be disruptive.
_Avoid_: pause, disable mode

**Meeting Reflection**:
A later review of whether a meeting was useful, what was understood, and what should happen next.
_Avoid_: meeting notes, transcript

**Drift Recovery**:
The forced return flow after Nagging Mode catches Attention Drift during a Focus Cycle.
_Avoid_: resume prompt, interruption

**Attention History**:
The local record of Focus Cycles and major attention events such as Nagging Mode and Meeting Mode.
_Avoid_: analytics, dashboard

**Attention Workflow**:
The domain module that owns the current attention phase, its workflow data, and the legal transitions between Attention Blocks, Focus Cycles, Review, and recovery flows.
_Avoid_: state machine, reducer, flow controller
