## Notes from Hacking Cryptography
*by Kamran Khan & Bill Cox*

### Chapter 2: Random Number Generators (RNGs)
---
1. Entropy is the measure of uncertainty in a system. 
2. For a fair coin toss system, the head or tail probabilities are equal, i.e. $p_{head} == p_{tail}$
3. Then entropy is 1 bit. Is this because we can use 1 bit to represent either states the coin might be after a toss - 0 or 1? With 0 and 1, we are equally unsure about the outcome of a toss. This is representational property.
> Uncertainty: We consider this an inherent property of the system, i.e. the system's unpredictability. While entropy is the measure of this property.
4. If we had 10 successive coin tosses, we'd need 10 bits to represent the entropy of the tosses.
5. For informational property, we need $log_2(1/p)$ to characterise the new information we get when we observe the outcome of an event, like the coin toss.