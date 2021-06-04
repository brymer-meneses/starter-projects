
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>



typedef struct Variable {
    double value;
    double grad;
    bool requires_grad;

    struct Variable *parent1;
    double v1_grad;
    struct Variable *parent2;
    double v2_grad;
} Variable;


Variable *create_variable(double value, bool requires_grad) {
    Variable* result = malloc(sizeof(Variable));
    if (result != NULL) {
        result->requires_grad = requires_grad;
        result->value = value;
        result->parent1 = NULL;
        result->parent2 = NULL;
    }
    return result;
}


Variable *mul(Variable v1, Variable v2) {
    double result = v1.value * v2.value;
    Variable* new_variable = malloc(sizeof(Variable));

    if (v1.requires_grad || v2.requires_grad) {
        new_variable->requires_grad = true;
    } else {
        new_variable->requires_grad = false;
    }

    if (v1.requires_grad) {
        new_variable->parent1 = &v1;
        new_variable->v1_grad = v2.value;
    }

    if (v2.requires_grad) {
        new_variable->parent2 = &v2;
        new_variable->v2_grad = v1.value;
    }

    return new_variable;
}

Variable *add(Variable v1, Variable v2) {
    double result = v1.value + v2.value;
    Variable* new_variable = malloc(sizeof(Variable));

    if (v1.requires_grad || v2.requires_grad) {
        new_variable->requires_grad = true;
    } else {
        new_variable->requires_grad = false;
    }

    if (v1.requires_grad) {
        new_variable->parent1 = &v1;
        new_variable->v1_grad = (double)1.0;
    }

    if (v2.requires_grad) {
        new_variable->parent2 = &v2;
        new_variable->v2_grad = (double)1.0;
    }

    new_variable->value = result;
    return new_variable;
}

void compute_grads(Variable v, double prev_grad) {
    Variable *dependents[2] = {v.parent1, v.parent2};
    double grads[2] = {v.v1_grad, v.v2_grad};

    for (int i=0; i<2; i++) {
        Variable *next_variable = dependents[i];
        double grad = grads[i];

        if (next_variable->requires_grad) {
            double this_variable_grads = prev_grad * grad;
            next_variable->grad = this_variable_grads + next_variable->grad;
            compute_grads(*next_variable, this_variable_grads);
        } else {
            continue;
        }

    };

}

void backprop(Variable v) {
    if (v.requires_grad == false) {
        printf("Called backprop on a variable that does not requires_grad");
        exit(0);
    };
    compute_grads(v, (double)1);
}

int main() {
    Variable *v2 = create_variable(1.0, true);
    Variable *v1 = create_variable(2.0, true);

    Variable *result = add(*v1, *v2);

    printf("%f",result->value);
    backprop(*result);
    return 0;
}
