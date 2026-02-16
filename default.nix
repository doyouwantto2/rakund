{
  pkgs ? import <nixpkgs> { },
}:

let
  # Libraries required at runtime (shared objects .so)
  runtimeLibs = with pkgs; [
    # Tauri / GUI deps
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

    # Audio / Hardware deps
    alsa-lib
    udev
  ];

  # Tools needed only during build time
  buildTools = with pkgs; [
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
  # nativeBuildInputs are for tools that run on the build host (like pkg-config)
  nativeBuildInputs = buildTools;

  # buildInputs are for libraries that the program links against
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
