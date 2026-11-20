# 발표자의 말 (영어 자막 복원)

오디오는 쓰지 않았습니다. [AgentOS 재게시본](../../agentos/ai-code-slop-poteto_ueA116hO_ho/)의 화면에 박힌 영어 자막을 프레임에서 판독해 시간순으로 모았습니다([../../agentos/ai-code-slop-poteto_ueA116hO_ho/transcripts/](../../agentos/ai-code-slop-poteto_ueA116hO_ho/transcripts/)).

- 시각은 **AgentOS 영상 기준**입니다. 채널이 중간중간 해설 카드를 끼워 넣어 원본 발표보다 약 5분 깁니다. 빈 구간(예: 2:34~3:16, 12:00~12:38)은 대부분 채널 해설입니다.
- 키프레임 간격(1~6초) 때문에 짧게 지나간 자막 줄은 빠졌을 수 있습니다. 문장이 끊긴 곳이 그런 자리입니다.
- 0:00~0:52는 채널의 도입부라 영어 자막이 없습니다.

```
[00:53] All right. So again, my name is poteto.
[00:56] I work on Grok Bot at SpaceXSI.
[00:59] And can I ask you a quick show of hands?
[01:02] How many of you have ever looked at code that somebody else wrote and you thought, that's all slop?
[01:10] Well, I have too.
[01:13] But today, I'm going to convince you that you can turn that slop into gold.
[01:21] And so I did something crazy last month, and I shipped 3,000 pull requests to production.
[01:26] Now, before agents, we never really cared about volume, except for the outliers.
[01:33] And at my previous company, we called these people coding machines.
[01:36] People like Bret Taylor, who famously rewrote all of Google Maps in a weekend.
[01:41] Just an insanely productive engineer.
[01:44] And well, with agents, anyone can now be a coding machine.
[01:48] And my thesis is that the job of the software engineer, the meta of being a software engineer,
[01:53] is figuring out how to maintain or even improve the quality of your code
[01:58] while massively increasing your productivity.
[02:02] To get there, some people like to call this a software factory.
[02:05] But I like to keep it simple, and I think of it as all about trust.
[02:11] Not blind trust, but investing in your agent's environment
[02:15] so you can trust that your agents are going to do the right thing.
[02:19] It takes quite a lot of effort to get there.
[02:22] And it took me six months to get to a point where I could ship thousands of pull requests, a lot of them in my sleep.
[02:29] But it's certainly possible, and it will take you much less time than I did.
[02:34] So let's start by talking about how we move from a low-trust environment to a high-trust one.
[03:16] So this inverse pyramid is how I think about where to spend time on your agent's environment.
[03:21] The first two layers are deterministic, so using your code itself.
[03:25] So things like good API design, architecture, refactoring,
[03:31] or using lint rules, type systems, compilers, or different kinds of testing.
[03:35] And you can use both of these layers together to constrain what agents can do
[03:38] and force them to do the right thing by default.
[03:43] Verification, both runtime and formal, are great tools to layer on
[03:49] to find bugs, and check properties and the invariants in your system that you want to hold true,
[03:55] and also help you check whether or not the agent understood your goals and your intent.
[03:59] And then finally, you want to layer with a little bit of prose,
[04:04] so things like rules, skills, AGENTS.md, things that require judgment.
[04:28] But let's start with your codebase.
[04:31] Agents generate their outputs based on their inputs.
[04:34] And so the quality of the input really matters.
[04:37] And it just so happens that your codebase is one of the largest inputs.
[04:42] So whenever you're correcting your agent, think about how you can make that a deterministic check
[04:47] instead of adding another line to your rules or your AGENTS.md,
[04:55] because it's far more reproducible when it's in code, and it lets you trust it more.
[04:57] And the analogy that I like to use is that your codebase is kind of like a garden.
[05:02] Code used to arrive a few pull requests at a time.
[05:06] And I remember going into work in 2024
[05:10] and reviewing the three pull requests that my colleagues wrote.
[05:14] And today, that number is more like 300.
[05:17] So every engineer is now a team of hundreds or even thousands of agents.
[05:23] And code is arriving so rapidly that it almost feels like a continuous stream.
[05:29] And it almost feels organic, like a living thing.
[05:33] And in this analogy, your codebase, the constraints that you impose, the guardrails that you put up
[05:40] are the concrete foundations that the new code that your agents write grow around,
[05:45] just like the building in this picture.
[05:48] And so in many ways, the job of being a software engineer is kind of becoming like gardening.
[05:57] Because code is the biggest input for agents,
[06:03] it means that any workarounds, anti-patterns, deprecated code, tech debt,
[06:10] it's now likely to be just copied by the next agent since it looks like the established pattern in your codebase.
[06:15] And it's like you have some weeds or pests growing in your garden,
[06:18] and then they spread everywhere before you know it.
[06:26] So you want to make your codebase as pristine as possible so that agents are always copying the good stuff.
[06:55] All right, let's look at some real examples.
[06:58] So Dune is a client framework that we created for Grok Bot
[07:02] to implement this idea for organic architecture.
[07:06] It's not really that interesting technically.
[07:08] You can think of Dune as kind of like Next.js for Electron apps.
[07:13] But the interesting part of Dune is that it has a lot of constraints and lint rules
[07:17] that we've encoded a lot of the lessons that we've learned building desktop applications.
[07:24] And one such lesson is that anything that requires nuance to use well often leads to slop.
[07:31] Your classic example is useEffect in React.
[07:35] It has its use cases, but it's often misused.
[07:39] Some people joke that you need a PhD to use useEffect.
[07:43] And in other words, it really requires a lot of nuance to use well.
[07:48] And agents are not good at using it.
[07:51] So our solution has been to just ban it in product code
[07:55] and it's really only allowed in some places inside of Dune APIs.
[08:00] And effectively, this problem has now been categorically eliminated,
[08:04] and we never need to catch that during code review again.
[08:08] Another interesting pattern that we use is context through directory structure.
[08:13] So one anti-pattern that we saw with Cursor, which is an Electron app,
[08:17] is that agents would often import main thread code into the render thread,
[08:22] which would cause long tasks and jank.
[08:25] So in Grok Bot and in Dune apps, we've made this very, very conventional.
[08:31] We have these two top-level directories, literally called electron-renderer and electron-main.
[08:36] And imports between them are forbidden or banned by our dependency graph analyzer and some lint rules.
[08:44] And again, these are simple lint rules to set up,
[08:46] but it means one less thing that we need to review and think about during code review.
[09:20] And then one more surprising thing that I've seen is agents using Markdown.
[09:24] So things like documentation, code comments, rules, or skills.
[09:28] And they use this as a place to write down their justifications for not doing the right thing.
[09:34] Or I don't know if you've ever seen this, but they'll over-rotate on some small correction that you made,
[09:39] and they treat it like a hard rule that can never be broken ever again.
[09:44] And here's another example from Cursor.
[09:47] So agents really loved using imperative timers,
[09:50] so things like setTimeout, requestAnimationFrame, requestIdleCallback,
[09:54] and they were using these as quick fixes because they were really patching over an architecture clash
[10:02] between VS Code's imperative style and React's declarative style.
[10:06] So our choice of architecture actually made both the client and the agent performance much worse.
[10:13] And if you've ever done any performance work in JavaScript, you'll know
[10:16] that when you have tons of timers being rapidly created and destroyed, it causes a lot of major garbage collection,
[10:22] which causes a lot of lag and jank.
[10:26] But the worst part is that agents wrote a lot of code comments, like you can see in red here,
[10:30] explaining why this quick fix was justified.
[10:34] And before you know it, we had timers spreading everywhere,
[10:38] and the performance of the app really suffered.
[10:49] Now, in Grok Bot, we learned our lesson there, and we decided to just ban 99% of comments
[10:51] because they go stale easily.
[10:55] Humans just really don't bother reading the comment slop anyway.
[10:59] And again, it gives an agent an escape hatch
[11:03] to justify not doing the right thing because it's hard.
[11:07] And the worst part about comments, though, is that they're not enforceable.
[11:13] So what seems well-intentioned, like, I'm going to tell the next agent not to do something,
[11:16] never ends up actually being anything that you can trust.
[11:23] So in Grok Bot and in Dune, we now have lint rules that give
[11:29] good and very helpful error messages telling the agent to fix the root cause, the real problem,
[11:31] rather than just patch around it.
[11:32] And so our general principle is that every time we correct our agents, it turns into one of these deterministic checks.
[11:38] Or we think about how to categorically eliminate that problem in the code through better API design, through better architecture.
[11:46] And if you do this enough times, you'll find that you have just less things to look out for in code review.
[11:53] And at some point, you'll realize there's nothing left to review.
[11:56] And you can just let agents merge most of their own pull requests.
[12:38] All right, moving on. Verification is also a really big part of how we make our agents more autonomous
[12:44] and also increase our confidence and trust in their work.
[12:48] And our code review process has now really shrunk into looking at verification artifacts instead.
[12:53] So things like videos, screenshots, diagrams, formal verification.
[12:59] And the most basic example of this is runtime verification,
[13:03] which is just a fancy way of saying giving your agent a way to run the code for itself
[13:07] and use debugging tools that human developers use as well,
[13:11] like the Chrome DevTools Protocol.
[13:14] In our runtime verification skills, we have this thing that we call a feature map.
[13:18] And that's basically kind of like a site map of all of the features that are available in your application from the user's point of view.
[13:26] And so with this, agents don't need to guess when you say the thingy in the left sidebar.
[13:31] It gives you some shared vocabulary.
[13:34] And we also make use of custom CLIs inside these skills so that agents can reuse the CLI and the driver every time,
[13:42] which cuts down on token usage and is also testable.
[14:17] And then recently, we've also started looking at formal verification.
[14:21] So we now use TLA+ to do some model checking of major user flows,
[14:25] such as notifications across multiple devices and platforms.
[14:30] We use Lean as well for proving certain algorithms.
[14:33] For example, we wrote our own custom virtualization library
[14:37] and we use Lean there to check that we don't have flickering or jittering problems.
[14:42] And we're still experimenting with this approach overall.
[14:45] It seems very promising.
[14:47] We've already found quite a few bugs with formal verification.
[14:52] But it's certainly not a silver bullet.
[14:54] And we're treating it as another tool,
[14:57] another layer in our agent's environment.
[15:02] And then this is a very fast video.
[15:04] But another thing that we just started looking at is property-based testing as well for UIs.
[15:11] So we use this library called Bombadil. And this has also caught a few UI bugs for us already.
[15:40] Okay, so zooming out, I saw this tweet from Cindy.
[15:43] And six months ago, she said that she was talking about not being able to trust her agents
[15:49] because of subtle bugs that were getting merged.
[15:54] And really what was missing for her in being able to trust her agents was the verification process.
[15:59] And now, six months later, she says that 70% of her daily tokens go into verification.
[16:04] And I'd say that tracks with what I see as well, that most of my token usage goes to verification.
[16:11] And if you really look closely at her process,
[16:13] none of these things here are new or novel techniques that we've never had before.
[16:19] These are just good software engineering tools,
[16:22] but ones that might have been prohibitively expensive to use before as they were hard to understand or write or maintain.
[16:29] But agents drastically cut this cost down.
[16:32] And really, we're entering a golden age of software testing and verification
[16:36] because agents let us test everything to a much greater degree than used to be economical.
[16:43] And then formal verification can also help you better reason about how your product should work
[16:48] and whether your agents really understand your intent and goals.
[16:51] I made this new skill called /statechart in my plugin pstack, poteto stack,
[16:57] which helps you visualize state machines from the user's point of view.
[17:02] And I find this much easier to review alongside the videos and screenshots that we provide.
[17:08] And so that's been great.
[17:38] Okay, now let's look at what the promised land looks like.
[17:41] What do you get in return for all of this investment?
[17:47] So on the Grok Bot team, our velocity is taking off and rapidly increasing.
[17:52] And last month, our team shipped a ridiculous 11,000 pull requests to production,
[17:59] which is a wild number.
[18:00] And our pace of releasing the client has also increased.
[18:04] And we cut a new stable version about once a day.
[18:08] And then the core team is also shipping much more.
[18:11] And we also have a lot more casual or drive-by contributors who don't always have a lot of context,
[18:19] but they're also contributing.
[18:21] And in October, we had about 227
[18:25] people who contributed at least one pull request.
[18:29] So despite this velocity accelerating, you might be thinking to yourself, isn't this going to be chaos?
[18:32] Isn't this going to be a bunch of slop?
[18:37] The number of PRs that get reverted has actually gotten lower on a relative basis
[18:41] and is steady at about half a percent.
[18:45] But I think the really interesting thing about this shipping pace and this volume of pull requests
[18:51] is that the performance of Grok Bot has stayed not just stable, but has improved on multiple dimensions compared to when we launched.
[18:59] So startup, typing, opening new chats, switching chats, all of this has gotten faster.
[19:05] And keep in mind, Grok Bot is still an Electron app for now.
[19:10] So there is a ceiling, right?
[19:12] But it's already much faster than many other agent UIs out there.
[19:17] And for an Electron app with this high velocity and this many people contributing, I'd say that's pretty good.
[19:48] So before you jump into building out a software factory and automating everything that you can,
[19:54] whenever you correct your agent, try and do it by improving your codebase first.
[19:58] Start with a lint rule to prevent new occurrences of anti-patterns.
[20:02] And then clean up those old patterns as well.
[20:05] And then make sure that your verification story is really strong.
[20:10] A lot of this work is a prerequisite to automation anyway.
[20:13] And if your inner coding loop produces slop, then automating that loop just produces more slop.
[20:20] And you need to throw more and more and more tokens at the problem, which we all know is very expensive.
[20:26] And once you meet these investments into your codebase, static analysis, runtime and formal verification,
[20:33] you'll start to find that many things become possible with these foundations,
[20:38] and that you can really easily build your own factory on top of these foundations.
[21:15] And as a quick example, whenever we cut a new release now,
[21:19] Grok Bot will automatically spin up a new Cursor project for us that will fuzz the entire application, the entire build,
[21:26] and it will find bugs by running the app and using it like a real user.
[21:30] And then it's going to open fix PRs for each of those bugs that it finds, and it's going to auto-merge it,
[21:37] as crazy as that sounds.
[21:38] And that's because our codebase is good enough that pull requests tend to just be good by default.
[21:43] And most product code that we land no longer really requires deep human review.
[21:50] So when you have solid foundations and good organic architecture, you can turn slop into gold,
[21:58] and you can ship with high quality, high velocity without compromising on quality.
[22:04] My name is poteto, also known as Lauren.
[22:06] Thanks to you again for listening to my talk.
```
