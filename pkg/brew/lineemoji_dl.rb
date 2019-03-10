class LineEmojiDL < Formula
  version 'v1.0.2'
  desc "download line emoji from url"
  homepage "https://github.com/laprasdrum/lineemoji_dl"

  if OS.mac?
      url "https://github.com/laprasdrum/lineemoji_dl/releases/download/#{version}/lineemoji_dl-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "LineEmojiDL"
  elsif OS.linux?
      url "https://github.com/laprasdrum/lineemoji_dl/releases/download/#{version}/lineemoji_dl-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "LineEmojiDL"
  end

  def install
    # Name of the binary
    bin.install "lineemoji_dl"
  end
end