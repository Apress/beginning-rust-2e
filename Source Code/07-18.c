/* It prints:
Result: 1*/
#include <stdio.h>
int main() {
    enum eResult {
        Success,
        Failure,
        Uncertainty
    };
    struct sResult {
        enum eResult r;
        union {
            char value;
            struct {
                unsigned short error_code;
                char module;
            } s;
        } u;
    } outcome;
    outcome.r = Success;
    outcome.u.value = 1;
    /*
    outcome.r = Failure;
    outcome.u.s.error_code = 20;
    outcome.u.s.module = 'X';
    */
    switch (outcome.r) {
        case Success:
            switch (outcome.u.value) {
                case 0:
                    printf("Result: 0");
                    break;
                case 1:
                    printf("Result: 1");
                    break;
                default:
                    printf("Result: other");
                    break;
            }
            break;
        case Failure:
            switch (outcome.u.s.error_code) {
                case 10:
                    switch (outcome.u.s.module) {
                        case 'X':
                            printf("Error: 10 X");
                            break;
                        default:
                            printf("Error: 10");
                            break;
                    }
                    break;
                default:
                    switch (outcome.u.s.module) {
                        case 'X':
                            printf("Error: X");
                            break;
                        default:
                            printf("Error: other");
                            break;
                    }
                    break;
            }
            break;
    case Uncertainty:
        break;
    }
    return 0;
}
