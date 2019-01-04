Name: {{{ git_dir_name }}}
Version: {{{ git_dir_version }}}
Release: 1%{?dist}
Summary: A wrapper around certbot with some extras ✨
License: MIT
ExclusiveArch:  %{rust_arches}
BuildRequires: rust-packaging
VCS: {{{ git_dir_vcs }}}
Source: {{{ git_dir_pack }}}

%description
%{summary}.

%prep
{{{ git_dir_setup_macro }}}
%cargo_prep

%build
%cargo_build

%install
%cargo_install
# Install systemd units
mkdir -p %{buildroot}%{_unitdir}
install -pm644 systemd-units/renew-certificates.service %{buildroot}%{_unitdir}
install -pm644 systemd-units/renew-certificates.timer %{buildroot}%{_unitdir}

%if %{with check}
%check
%cargo_test
%endif

# %changelog
# {{{ git_dir_changelog }}}

%license LICENSE-MIT
%doc readme.md

%files
%{_bindir}/certbot-wrapper
