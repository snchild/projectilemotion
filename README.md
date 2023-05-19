# Overview

I am working on learning the basics of Rust by writing software that solves a physics projectile motion problem. The purpose of this software is ultimately to become familiar with the syntax that Rust uses and to develop the skill of easily learning new languages.

This software first prompts the user for an angle and velocity that a projectile is launched at. It then calculates the horizontal distance that the projectile travels, as well as the time that the projectile is in the air. It also takes note of the greatest distance and the greatest time. This is significant because the software does all that is listed before many times, until the user enters "q" instead of a number. Once the user has entered "q", the software displays the farthest distance and the longest times.  

The physics used in this software is basic classical mechanics, particularly kinematics. This software ignores air drag and assumes that the inputted values are highly accurate. 

[Software Demo Video](https://youtu.be/gDld_-O5_C8)

# Development Environment

I used Visual Studio and Rust to develop this software. 

The equations that I used to calculate the values in this code are from the kinematic equations, namely $$x_f = x_i + v_i t + \frac{1}{2} a t^2$$ I have decided that the origin will be at the launch position, so the equation of motion for x is $x_f = v_0 cos(\theta) t$ and the equation of motion for y is $y_f = v_0 sin(\theta) t - \frac{1}{2} g t^2$. 
If we combine these two equations of motion and solve for $x_f$, the horizontal distance the projectile travels, the resulting equation is $x_f = \frac{v^2 sin(2\theta)}{g}$

# Useful Websites

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [Rust-Lang](https://doc.rust-lang.org/std/primitive.array.html)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/flow_control/for.html)
- [Stack Overflow](https://stackoverflow.com/questions/48634335/how-do-i-multiply-an-integer-and-a-floating-value-together-and-display-the-resul)
- [Stack Overflow](https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0/30355925#30355925)

# Future Work

- Add a plotting function to display the trajectory of the projectile.
- Add a maximum vertical distance as if the projectile were launched indoors and calculate the farthest horizontal distance taking that maximum vertical distance into account. 
- Modify the convert_to_float function to return -1.0 if the user enters "q" and also displays an error message then prompts the user again if they enter something that is not a number or "q"