#include <stdio.h>
#include <string.h>
#include <stdlib.h>

char *toalphanumeric(char *input, int trigger, int onlyalnum) {
	int length;
	int increment;
	int index;
	char *part;
	char character;

	length = 1;

	char *ret = calloc(length, 1);

	for (index = 0; input[index] != '\0'; index++) {
		character = input[index];

		if ((character >= 'a' && character <= 'z') || (character >= '0' && character <= '9')) {
			ret = realloc(ret, length + 1);
			ret[length-1] = character;
			ret[length] = '\0';
			length++;
		} else if (character >= 'A' && character <= 'Z') {
			ret = realloc(ret, length + 1);
			if (trigger != 0)
				ret[length-1] = (character | 0x20);
			else
				ret[length-1] = character;
			ret[length] = '\0';
			length++;
		} else if (trigger == 0) {
			if (character != '`') {
				ret = realloc(ret, length + 1);
				ret[length-1] = character;
				ret[length] = '\0';
				length++;
			}
		} else if (character == ' ' || character == '\n') {
			ret = realloc(ret, length + 1);
			ret[length-1] = character;
			ret[length] = '\0';
			length++;
		} else if (onlyalnum == 1) {
			if (character == '-' || character == '/') {
				ret = realloc(ret, length + 1);
				ret[length-1] = ' ';
				ret[length] = '\0';
				length++;
			}
			continue;
		} else if (character == '~') {
			part = strdup("the tilde character ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '`') {
			part = strdup("the backtick character ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '!') {
			part = strdup("the exclamation point ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '@') {
			part = strdup("the at sign ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '#') {
			part = strdup("the pound sign ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '$') {
			part = strdup("the dollar sign ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '%') {
			part = strdup("the percent sign ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '^') {
			part = strdup("the caret character ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '&') {
			part = strdup("the ampersand character ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '*') {
			part = strdup("the asterisk character ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '(') {
			part = strdup("the open parenthesis character ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == ')') {
			part = strdup("the close parenthesis character ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '-') {
			part = strdup("the hyphen character ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '_') {
			part = strdup("the underscore character ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '+') {
			part = strdup("the plus sign ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '=') {
			part = strdup("the equal sign ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '[') {
			part = strdup("the open square bracket ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == ']') {
			part = strdup("the close square bracket ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '{') {
			part = strdup("the open curly bracket ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '}') {
			part = strdup("the close curly bracket ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '/') {
			part = strdup("the forward slash ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '\\') {
			part = strdup("the backslash ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '|') {
			part = strdup("the pipe character ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == ';') {
			part = strdup("the semicolon ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == ';') {
			part = strdup("the colon ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '\'') {
			part = strdup("the apostrophe ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '"') {
			part = strdup("the quotation mark ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '<') {
			part = strdup("the less than character ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '>') {
			part = strdup("the greater than character ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == ',') {
			part = strdup("the comma ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '.') {
			part = strdup("the period ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		} else if (character == '?') {
			part = strdup("the question mark ");
			increment = strlen(part);
			ret = realloc(ret, length + increment);
			snprintf(&ret[length-1], increment+1, "%s", part);
			length += increment;
			free(part);
		}
	}

	if (length > 2 && ret[length-3] == ' ') {
		ret[length-3] = '\n';
		ret[length-2] = '\0';
	}

	return ret;
}

char *fmtexample(char *input) {
	char *ret;
	int index;
	int length;
	char character;

	length = 1;

	ret = calloc(length, 1);

	for (index = 0; input[index] != '\0'; index++) {
		character = input[index];

		switch(character) {
		case '`':
			break;
		case '{':
			if (input[index+1] == '{') {
				index++;
				break;
			}
		case '}':
			if (input[index+1] == '}') {
				index++;
				break;
			}
		default:
			ret = realloc(ret, length + 1);
			ret[length-1] = character;
			ret[length] = '\0';
			length++;
			break;
		}
	}

	return ret;
}

int main(int argc, char **argv) {
	char buffer[8192];
	char *pointer;
	char *pointer2;
	int line;
	char *cmdname;

	line = 1;

	if (fgets(buffer, sizeof(buffer), stdin) == NULL) {
ERROR1:
		fprintf(stderr, "error: immediate end of file at line %d\n", line);
		return -1;
	}

	if (strncmp(buffer, "# ", 2) != 0) {
ERROR2:
		fprintf(stderr, "error: tldr markdown format mismatch at line %d\n", line);
		return -1;
	}

	cmdname = strdup(&buffer[2]);
	cmdname[strcspn(cmdname, "\n")] = '\0';

	pointer = toalphanumeric(cmdname, 1, 0);
	printf("+ %s\n", pointer);
	free(pointer);

	line = 2;

	if (fgets(buffer, sizeof(buffer), stdin) == NULL)
		goto ERROR1;

	if (strcmp(buffer, "\n") != 0)
		goto ERROR2;

	line = 3;

	if (fgets(buffer, sizeof(buffer), stdin) == NULL)
		goto ERROR1;

	if (strncmp(buffer, "> ", 2) != 0)
		goto ERROR2;

	pointer = toalphanumeric(&buffer[2], 0, 0);
	printf("- %s: %s", cmdname, pointer);

	while(fgets(buffer, sizeof(buffer), stdin) != NULL) {
		line++;
		if (strncmp(buffer, "> ", 2) != 0)
			break;
	}

	if (strcmp(buffer, "\n") != 0)
		goto ERROR2;

	printf("%s", buffer);

	pointer2 = toalphanumeric(pointer, 1, 1);
	printf("+ %s- The %s command.\n\n", pointer2, cmdname);

	free(pointer2);
	free(pointer);

	while(fgets(buffer, sizeof(buffer), stdin) != NULL) {
		line++;
		if (strncmp(buffer, "- ", 2) == 0) {
			pointer = toalphanumeric(&buffer[2], 1, 1);
			printf("+ %s", pointer);
			free(pointer);

			while(fgets(buffer, sizeof(buffer), stdin) != NULL) {
				line++;
				if (buffer[0] == '`') {
					pointer = fmtexample(buffer);
					printf("- This command: %s\n", pointer);
					free(pointer);

					break;
				}
			}
		}
	}
}

