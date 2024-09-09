#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_VAR_NAME 64
#define MAX_VAR_VALUE 256
#define MAX_VARS 100
#define MAX_FUNC_NAME 64
#define MAX_FUNC_BODY 1024
#define MAX_FUNCS 10

void process_line(const char *line);
void process_file(const char *filename);
void set_var(const char *name, const char *value);
const char* get_var(const char *name);
void delete_var(const char *name);
void define_function(const char *name, const char *body);
void execute_function(const char *name);
void replace_variables(char *str);
double evaluate_expression(const char *expr);
void process_escape_sequences(char *line);

typedef struct {
    char name[MAX_VAR_NAME];
    char value[MAX_VAR_VALUE];
} Variable;

typedef struct {
    char name[MAX_FUNC_NAME];
    char body[MAX_FUNC_BODY];
} Function;

Variable vars[MAX_VARS];
Function funcs[MAX_FUNCS];
int var_count = 0;
int func_count = 0;

// Function to evaluate simple arithmetic expressions
double evaluate_expression(const char *expr) {
    double result = 0.0;
    char operator = '+';
    const char *p = expr;

    while (*p) {
        while (isspace(*p)) ++p; // Skip whitespace
        if (isdigit(*p) || *p == '-') {
            double value = strtod(p, (char **)&p);
            if (operator == '+') result += value;
            else if (operator == '-') result -= value;
            else if (operator == '*') result *= value;
            else if (operator == '/') result /= value;
        } else if (*p == '+' || *p == '-' || *p == '*' || *p == '/') {
            operator = *p++;
        } else {
            ++p;
        }
    }
    return result;
}

// Function to replace variable placeholders in strings (e.g., $a with the value of a)
void replace_variables(char *str) {
    char buffer[MAX_VAR_VALUE * 2] = {0};
    char var_name[MAX_VAR_NAME] = {0};
    const char *p = str;
    char *buf_ptr = buffer;

    while (*p) {
        if (*p == '$' && isalpha(*(p + 1))) {
            p++; // Skip the '$'
            char *var_ptr = var_name;
            while (isalnum(*p)) {
                *var_ptr++ = *p++;
            }
            *var_ptr = '\0'; // Null-terminate the variable name

            // Get the variable value
            const char *var_value = get_var(var_name);
            if (var_value) {
                strcpy(buf_ptr, var_value);
                buf_ptr += strlen(var_value);
            } else {
                printf("Error: Undefined variable %s\n", var_name);
                return;
            }
        } else {
            *buf_ptr++ = *p++;
        }
    }
    *buf_ptr = '\0'; // Null-terminate the buffer
    strcpy(str, buffer); // Copy the modified string back
}

// Function to set a variable
void set_var(const char *name, const char *value) {
    for (int i = 0; i < var_count; ++i) {
        if (strcmp(vars[i].name, name) == 0) {
            strncpy(vars[i].value, value, MAX_VAR_VALUE);
            return;
        }
    }
    if (var_count < MAX_VARS) {
        strncpy(vars[var_count].name, name, MAX_VAR_NAME);
        strncpy(vars[var_count].value, value, MAX_VAR_VALUE);
        ++var_count;
    } else {
        printf("Error: Too many variables\n");
    }
}

// Function to get the value of a variable
const char* get_var(const char *name) {
    for (int i = 0; i < var_count; ++i) {
        if (strcmp(vars[i].name, name) == 0) {
            return vars[i].value;
        }
    }
    return NULL;
}

// Function to delete a variable
void delete_var(const char *name) {
    for (int i = 0; i < var_count; ++i) {
        if (strcmp(vars[i].name, name) == 0) {
            for (int j = i; j < var_count - 1; ++j) {
                vars[j] = vars[j + 1];
            }
            --var_count;
            return;
        }
    }
    printf("Error: Undefined variable %s\n", name);
}

// Function to define a function
void define_function(const char *name, const char *body) {
    for (int i = 0; i < func_count; ++i) {
        if (strcmp(funcs[i].name, name) == 0) {
            strncpy(funcs[i].body, body, MAX_FUNC_BODY);
            return;
        }
    }
    if (func_count < MAX_FUNCS) {
        strncpy(funcs[func_count].name, name, MAX_FUNC_NAME);
        strncpy(funcs[func_count].body, body, MAX_FUNC_BODY);
        ++func_count;
    } else {
        printf("Error: Too many functions\n");
    }
}

// Function to execute a function
void execute_function(const char *name) {
    for (int i = 0; i < func_count; ++i) {
        if (strcmp(funcs[i].name, name) == 0) {
            char body[MAX_FUNC_BODY];
            strcpy(body, funcs[i].body);
            replace_variables(body);
            printf("%s\n", body); // Print the processed function body
            return;
        }
    }
    printf("Error: Undefined function %s\n", name);
}

// Function to read and process commands from a file
void process_file(const char *filename) {
    FILE *file = fopen(filename, "r");
    if (!file) {
        printf("Error: Could not open file %s\n", filename);
        return;
    }

    char line[MAX_VAR_VALUE];
    while (fgets(line, sizeof(line), file)) {
        line[strcspn(line, "\n")] = '\0'; // Remove trailing newline
        process_line(line);
    }

    fclose(file);
}

// Function to process a line of code
void process_line(const char *line) {
    if (line[0] == '.') {
        // Comment line
        return;
    }

    // Is line blank?
    if (line[0] == '\0') {
        return;
    }

    if (strncmp(line, "var ", 4) == 0) {
        char name[MAX_VAR_NAME];
        char value[MAX_VAR_VALUE];
        if (sscanf(line + 4, "%[^=]=%s", name, value) == 2) {
            replace_variables(value);
            if (strchr(value, '+') || strchr(value, '-') || strchr(value, '*') || strchr(value, '/')) {
                double result = evaluate_expression(value);
                snprintf(value, MAX_VAR_VALUE, "%lf", result);
            }
            set_var(name, value);
        } else {
            fprintf(stderr, "Error: Invalid variable declaration format.\n");
        }
        return;
    }

    if (line[0] == '-') {
        delete_var(line + 1);
        return;
    }

    if (strncmp(line, "if ", 3) == 0) {
        // Handle basic if statements (e.g., "if a>5")
        char condition[MAX_VAR_VALUE];
        sscanf(line + 3, "%s", condition);
        double result = evaluate_expression(condition);
        if (result != 0) {
            printf("Condition met, execute next line\n");
        }
        return;
    }

    if (strncmp(line, "func ", 5) == 0) {
        // Define a function (e.g., "func myFunc print(a)")
        char name[MAX_FUNC_NAME];
        char body[MAX_FUNC_BODY];
        sscanf(line + 5, "%s %[^\n]", name, body);
        define_function(name, body);
        return;
    }

    if (strncmp(line, "call ", 5) == 0) {
        // Call a function (e.g., "call myFunc")
        char name[MAX_FUNC_NAME];
        sscanf(line + 5, "%s", name);
        execute_function(name);
        return;
    }

    if (strncmp(line, "input ", 6) == 0) {
        // Read input from user (e.g., "input a")
        char name[MAX_VAR_NAME];
        sscanf(line + 6, "%s", name);
        // Custom message to display to the user (e.g., "input a -> Type your name: ")
        char message[MAX_VAR_NAME + 32];
        char value[MAX_VAR_VALUE];
        // Check if a custom message is provided
        if (sscanf(line + 6 + strlen(name) + 1, " -> %[^\n]", message) == 1) {
            printf("%s", message);
        } else {
            printf("Enter value for %s: ", name);
        }
        if (fgets(value, sizeof(value), stdin) != NULL) {
            value[strcspn(value, "\n")] = '\0'; // Remove trailing newline
            set_var(name, value);
        } else {
            fprintf(stderr, "Error reading input\n");
        }
        return;
    }

    // Fall back to printing the line. ANSI C escape sequences are supported.
    char processed_line[MAX_VAR_VALUE];
    strcpy(processed_line, line);
    replace_variables(processed_line);
    process_escape_sequences(processed_line);
    printf("%s\n", processed_line);
}

void process_escape_sequences(char *line) {
    char *src = line;
    char *dst = line;
    while (*src) {
        if (*src == '\\' && *(src + 1) == '0' && *(src + 2) == '3' && *(src + 3) == '3') {
            *dst++ = '\033';
            src += 4;
        } else if (*src == '\\' && *(src + 1) == 'x') {
            char hex[3] = { *(src + 2), *(src + 3), '\0' };
            *dst++ = (char)strtol(hex, NULL, 16);
            src += 4;
        } else {
            *dst++ = *src++;
        }
    }
    *dst = '\0';
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Usage: %s <file>\n", argv[0]);
        return 1;
    }

    process_file(argv[1]);

    return 0;
}