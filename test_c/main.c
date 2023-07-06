#include <stdio.h>
#include <stdlib.h>
#include <oqs/oqs.h>

void print_array(uint8_t *array, size_t size) {
    for (size_t i = 0; i < size; i++) {
        printf("%02x ", array[i]);
    }
    printf("\n");
}

int main() {
    OQS_STATUS rc;

    // Initialisation
    OQS_SIG *sig = OQS_SIG_new(OQS_SIG_alg_falcon_512);
    if (sig == NULL) {
        printf("Falcon n'est pas supporté par cette installation de liboqs.\n");
        return EXIT_FAILURE;
    }

    // Génération des clés
    uint8_t *public_key = malloc(sig->length_public_key);
    uint8_t *secret_key = malloc(sig->length_secret_key);
    rc = OQS_SIG_keypair(sig, public_key, secret_key);
    if (rc != OQS_SUCCESS) {
        printf("Génération des clés a échoué.\n");
        return EXIT_FAILURE;
    }

    print_array(public_key, sig->length_public_key);
    print_array(secret_key, sig->length_secret_key);

    printf()
    // Nettoyage
    free(public_key);
    free(secret_key);
    OQS_SIG_free(sig);

    return EXIT_SUCCESS;
}
