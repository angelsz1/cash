<h1>Cash Shell</h1>

<p>Cash is a Unix shell built in Rust programming language. The goal of Cash is to provide a simple and efficient shell.</p>

<h2>Installation</h2>

<p>To install Cash, you'll need Rust and Cargo, Rust's package manager, installed on your machine. Once you have Rust and Cargo installed, run the following commands:</p>

<pre><code>$ git clone https://github.com/angelsz1/cash.git
$ cargo install --path cash
</code></pre>

<p>This will download the Cash source code, compile it, and install the binary on your system. After installation, you should be able to run the <code>cash</code> command in your terminal.</p>

<h2>Usage</h2>

<p>To run the Cash shell, simply type <code>cash</code> in your terminal. Cash will start a new session and display a prompt, where you can enter commands.</p>

<p>Cash supports a variety of Unix shell commands and syntax. Here are some examples:</p>

<pre><code>cash $ ls
cash $ cd /path/to/directory
cash $ echo "Hello, world!"
cash $ cat filename.txt | grep "search term" <---- pipes not yet supported
</code></pre>

<p>That's it! You're ready to start using the Cash shell.</p>

<h2>You can also set custom aliases!</h2>
<p>To do so, just go to <code>/home/[your_user]/.cashrc.toml</code> and add one in the "Aliases" Table </p>
<p>For example:</p>
<pre><code>[aliases]
gs = "git status"
</code></pre>
