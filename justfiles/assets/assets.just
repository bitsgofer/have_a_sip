# == REFERNECE ASSETS ==========================================================
# Manage reference/mockup assets

# Track a dowloaded reference assets and add a TODO entry in CREDITS.md
add-references name source:
    ./add-references.bash "{{ name }}" "{{ source }}"
