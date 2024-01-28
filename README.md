# 2023 Advent of Code

This repository contains my solutions to the [Advent of Code 2023](https://adventofcode.com/2023) challenges. I did this to explore using Rust, and explore what using a new-ish language would be like when using a tool like Github Co-Pilot.

## Final Thoughts

Advent of code is kind of fun, but also kind of like doing Uni assignments for fun. I'm not sure if it's really for me. 

### Thoughts on using Co-Pilot with a new language

Co-Pilot is amanzing for getting over the syntax bump when using a new language. It doesn't always get it right, but it's a lot quicker can than googling docs and stack overflow. You can also use co-pilot to help you learn by asking questions about syntax, but I'm not sure how good the answers always are.

### Thoughts on Rust

Rust is a really interesting language. The extra safety it gives you obviously comes with some extra complexity. Advent of code is also not a use case where that extra safety is needed. As a result there was a few times where I was wishing I could go back to more familiar javascript.

I didn't entirely grasp the concept of ownership, and lifetimes are almost a complete mystery. Co-pilot was interesting here as it helped me fumble through these areas, but it also got it wrong quite often. Thankfully the compiler is also great.

When doing string manipulation, similar tasks in Javascript would give me the "this feels wrong" feeling, usually because I get lazy and don't write all the required checks But in Rust it is easy to write code that feels super safe, with all the `.expect` etc.

