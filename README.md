<h1>Cash Shell</h1>

<p>Cash is a Unix shell built in Rust programming language. The goal of Cash is to provide a simple and efficient shell that can be easily extended and customized. Cash has features such as command-line history, tab completion, piping, and redirection, and it supports custom plugins written in Rust.</p>

<h2>Installation</h2>

<p>To install Cash, you'll need Rust and Cargo, Rust's package manager, installed on your machine. Once you have Rust and Cargo installed, run the following command:</p>

<pre><code>$ cargo install cash</code></pre>

<p>This will download the Cash source code, compile it, and install the binary on your system. After installation, you should be able to run the <code>cash</code> command in your terminal.</p>

<h2>Usage</h2>

<p>To run the Cash shell, simply type <code>cash</code> in your terminal. Cash will start a new session and display a prompt, where you can enter commands.</p>

<pre><code>cash $</code></pre>

<p>Cash supports a variety of Unix shell commands and syntax. Here are some examples:</p>

<pre><code>cash $ ls
cash $ cd /path/to/directory
cash $ echo "Hello, world!"
cash $ cat filename.txt | grep "search term" <---- pipes not yet supported
</code></pre>

<p>That's it! You're ready to start using the Cash shell.</p>
