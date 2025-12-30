linters <- linters_with_defaults(
    line_length_linter(120L),
    indentation_linter(indent = 4L, hanging_indent_style = "always"),
    commented_code_linter = NULL
)
