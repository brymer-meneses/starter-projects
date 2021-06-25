
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>

typedef struct node {
    struct variable *var;
    double grad;
} node;

typedef struct variable {
    double value;
    double grad;
    bool requires_grad;
    node *parent1;
    node *parent2;
} variable;

variable *_init_variable(double value, bool requires_grad, node *parent1, node *parent2) {
    variable *new_variable = malloc(sizeof *new_variable);

    if (new_variable) {
        new_variable->value = value;
        new_variable->requires_grad = requires_grad;
        new_variable->parent1 = parent1;
        new_variable->parent2 = parent2;

        if (requires_grad) {
            new_variable->grad = 0.0;
        } else {
            new_variable->grad = NAN;
        }
    }
    return new_variable;
}

variable *mul(variable *v1, variable *v2) {
    double result = v1->value * v2->value;
    bool requires_grad = v1->requires_grad || v2->requires_grad;
    node *p1 = NULL;
    node *p2 = NULL;

    if (v1->requires_grad) {
        p1 = malloc(sizeof *p1);
        if (p1) {
            p1->var = v1;
            p1->grad = v2->value;
        } else{
            printf("Mem Alloc Failure");
            exit(1);
        }
    } 
    if (v2->requires_grad) {
        p2 = malloc(sizeof *p2);
        if (p2) {
            p2->var = v2;
            p2->grad = v1->value;
        } else{
            printf("Mem Alloc Failure");
            exit(1);
        }
    }

    return _init_variable(result, requires_grad, p1, p2);
}

variable *add(variable *v1, variable *v2) {
    double result = v1->value + v2->value;
    bool requires_grad = v1->requires_grad || v2->requires_grad;
    node *p1 = NULL;
    node *p2 = NULL;

    if (v1->requires_grad) {
        p1 = malloc(sizeof *p1);
        if (p1) {
            p1->var = v1;
            p1->grad = 1.0;
        } else{
            printf("Mem Alloc Failure");
            exit(1);
        }
    } 
    if (v2->requires_grad) {
        p2 = malloc(sizeof *p2);
        if (p2) {
            p2->var = v2;
            p2->grad = 1.0;
        } else{
            printf("Mem Alloc Failure");
            exit(1);
        }
    }

    return _init_variable(result, requires_grad, p1, p2);
}

void _calculate_grad(variable *v, double grad) {
    if (v->parent1 != NULL) {
        variable *next_variable = v->parent1->var;
        if (next_variable->requires_grad) {
            node *p1 = v->parent1;
            double this_variable_grads = grad * p1->grad;
            next_variable->grad += this_variable_grads;
            _calculate_grad(next_variable, this_variable_grads);
        }
    }
    
    if (v->parent2 != NULL) {
        variable *next_variable = v->parent2->var;
        if (next_variable->requires_grad) {
            node *p2 = v->parent2;
            double this_variable_grads = grad * p2->grad ;
            next_variable->grad += this_variable_grads;
            _calculate_grad(next_variable, this_variable_grads);
        }
    }
}

variable *var(double value, bool requires_grad) {
    return _init_variable(value, requires_grad, NULL, NULL);
}

void backprop(variable *v1) {
    _calculate_grad(v1, 1.0);
}

int main() {
    variable *x = var(5.0, true);
    variable *y = var(4.0, true);
    variable *z = var(10.0, true);

    variable *a = mul(x, y);
    variable *b = mul(a, x);

    backprop(b);

    // a = xy
    // b = xa
    //   = x(xy)
    //   == x^2 y
    // dB/dx = 2xy
    //       = 2(5)(4)
    //       = 40

    printf("%f\n", x->value);
    printf("%f\n", x->grad); // 40
}

