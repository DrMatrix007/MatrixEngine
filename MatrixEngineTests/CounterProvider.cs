using MatrixEngine.StateManagment;

namespace MatrixEngine {
    public class CounterProvider : StateProvider<int> {
        public int data { get; set; } = 0;

        public CounterProvider() : base(0) {
        }

        public void Add() {
            this.OperateState((i) => i + 1);
        }

        public new int Get() {
            return data;
        }

    }
    
}
