﻿namespace MatrixEngine.GameObjects.StateManagment {
    public class Consumer<Output> {

        public Provider<Output> provider
        {
            get;
        }

        public Consumer(Provider<Output> provider) {
            this.provider = provider;
        }




    }
}
