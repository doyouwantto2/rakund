{
  pkgs ? import <nixpkgs> { },
}:

let
  runtimeLibs = with pkgs; [
    gtk3
    gdk-pixbuf
    cairo
    pango
    libsoup_3
    webkitgtk_4_1
    gvfs
    glib
    mesa
    libglvnd
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
    gst_all_1.gst-plugins-good

    alsa-lib
    udev
  ];

  buildTools = with pkgs; [
    # rustup
    pkg-config
    git
    nodejs
    bun
    tailwindcss
    xdg-utils
    makeWrapper
  ];

in
pkgs.mkShell {
  nativeBuildInputs = buildTools;

  buildInputs = runtimeLibs;

  shellHook = ''
    # Combine all runtime libraries into LD_LIBRARY_PATH so the binary can find them
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath runtimeLibs}:$LD_LIBRARY_PATH"

    # Help pkg-config find the .pc files for alsa, openssl, webkit, etc.
    export PKG_CONFIG_PATH="${
      pkgs.lib.makeSearchPathOutput "dev" "lib/pkgconfig" runtimeLibs
    }:$PKG_CONFIG_PATH"

    # Ensure XDG utilities (like xdg-open) are available
    export PATH="$PATH:${pkgs.xdg-utils}/bin"

    # Fix for WebKit rendering issues in some Nix environments
    export WEBKIT_DISABLE_COMPOSITING_MODE=1

    echo "--- Audio & Tauri Dev Environment Loaded ---"
    echo "ALSA and UDEV paths have been exported to LD_LIBRARY_PATH."
  '';
}
