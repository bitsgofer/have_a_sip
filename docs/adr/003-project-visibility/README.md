# Move HaS repo to a private repo

An ideal I pursue is to eventually release the code for `Have a sip` (HaS).

This is because HaS is much closer to a system to author stories than a game.
Here, we don't just write stories, we also design the character's portrait,
silhouette, their music motif, interactions, etc.

For more people to author, the barrier of entry must be easier. And ultimately,
being able to read the source code ensure a minimum level of accessibility

So initially, HaS source code was in a public repo. However, I have now decided
to move the source code to a private repo. The reason is:

- **Showing too much, too early**: A lot of stuff in HaS is hacky right now.
  They might create the wrong impressions (e.g: sloppy code).
- **Getting a lead in creation**: I might hallucinate this one, but I definitely
  don't want somebody to fork HaS now and run it in some other direction.
  But rather than a human, it's more likely that some AI will scoop the code up.
  And although it is hypocritical (as I do use AI bots), I want to retain strong
  control here, at least for now.
- **Not paying more to Github**: This is the strongest motivator that drives
  the move. After trying to make HaS private and onboard a friend to work on it,
  I realized that Github charges a seat for each private repo's contributor.
  Even at $5 USD/user/month, it's not a price I'm willing to pay as all of us
  only work on this on the weekend.

# Decision

I will move HaS repo to <https://github.com/exklamationmark/have_a_sip>.

This should be pretty seamless. All we need to do is change `origin`.

The main benefit here is that I can add unlimited number of collaborators
while keeping the repo private.
