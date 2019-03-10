class LineemojiDl < Formula
  version 'v1.0.2'
  desc "download line emoji from url"
  homepage "https://github.com/laprasdrum/lineemoji_dl"

  if OS.mac?
      url "https://github.com/laprasdrum/lineemoji_dl/releases/download/#{version}/lineemoji_dl-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "53c643eb8b2ff9fe7ea7056168710d11d4e157c4fc2c06a24249a0590515d6c4"
  elsif OS.linux?
      url "https://github.com/laprasdrum/lineemoji_dl/releases/download/#{version}/lineemoji_dl-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "5eb6aff872f4e0934491ef0467fe902bd73e25c580921667e4dab28f5a170db5"
  end

  def install
    # Name of the binary
    bin.install "lineemoji_dl"
  end
end