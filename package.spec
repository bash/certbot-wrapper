Name: certbot-wrapper
Version: 0.1.0
Release: 1%{?dist}
Summary: A wrapper around certbot with some extras ✨
License: MIT

BuildRequires: rust-packaging
VCS: {{{ git_dir_vcs }}}
Source: {{{ git_dir_pack }}}

%description
%{summary}.

%prep
{{{ git_dir_setup_macro }}}

%build
%cargo_build

%install
%cargo_install

# %changelog
# {{{ git_dir_changelog }}}

%license LICENSE-MIT
%doc readme.md
