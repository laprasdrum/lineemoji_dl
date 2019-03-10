class LineemojiDl < Formula
  version 'v1.0.3'
  desc "download line emoji from url"
  homepage "https://github.com/laprasdrum/lineemoji_dl"

  if OS.mac?
      url "https://github.com/laprasdrum/lineemoji_dl/releases/download/#{version}/lineemoji_dl-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "6e9ef3b5d072198f4acd79b74b858c26732604a77fd03cb3532f9e1dfc51afa6"
  elsif OS.linux?
      url "https://github.com/laprasdrum/lineemoji_dl/releases/download/#{version}/lineemoji_dl-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "6d5c20fb2e53b636ec8153ce88925e02de2f38ea26e4ee9e849dc1ba858a67cd"
  end

  def install
    # Name of the binary
    bin.install "lineemoji_dl"
  end
end