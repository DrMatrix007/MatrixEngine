using System;

namespace MatrixEngine.StateManagment {
    public class ProviderConverter<Output,Input> : Provider<Output> {
        public ProviderConverter(Provider<Input> baseprovider, Func<Input, Output> func) {
            this.baseprovider = baseprovider;
            this.func = func;
        }

        private Provider<Input> baseprovider;

        private Func<Input, Output> func;
        
        Output Provider<Output>.data { get; set; }

        public Output Get() {
            return func(baseprovider.Get());
        }
    }
}