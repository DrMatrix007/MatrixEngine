using MatrixEngine.GameObjects.Components;
using MatrixEngine.MathM;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.Physics {
    public class PhysicsEngine {

        private List<RigidBodyComponent> rigidBodies;

        public App.App app
        {
            get;
            private set;
        }

        public PhysicsEngine(App.App app) {
            this.app = app;
            rigidBodies = new List<RigidBodyComponent>();
        }

        public void AddToFrameComputing(RigidBodyComponent rigidBodyComponent) {

            rigidBodies.Add(rigidBodyComponent);


        }
        public void Update() {

            foreach (var item in rigidBodies) {
                if (!item.isStatic) {

                    item.velocity += item.gravity;

                    item.position += item.velocity * app.deltaTime;

                    item.velocity.X = MathUtils.LerpToZero(item.velocity.X, item.velocityDrag.X);
                    item.velocity.Y = MathUtils.LerpToZero(item.velocity.Y, item.velocityDrag.Y);
                }
            }

            //work
            for (int i = 0; i < rigidBodies.Count; i++) {
                for (int x = 0; x < rigidBodies.Count; x++) {
                    if (x > i) {
                        var objectA = rigidBodies.ElementAt(x);
                        var objectB = rigidBodies.ElementAt(i);

                        RigidBodyComponent nonstatic;
                        RigidBodyComponent @static;

                        if ((objectA.isStatic && objectB.isStatic) || ((!objectA.isStatic) && (!objectB.isStatic))) {
                            continue;
                        }

                        if (objectA.isStatic) {
                            @static = objectA;
                            nonstatic = objectB;

                        } else {
                            @static = objectB;
                            nonstatic = objectA;
                        }

                        var result = nonstatic.rect.GetCollidingFixFromB(@static.rect);

                        if (result.axis == Physics.CollidingAxis.X) {
                            var pos = nonstatic.position;
                            pos.X -= result.fixValue;

                            nonstatic.position = pos;

                            nonstatic.velocity.X = 0;

                        }
                        if (result.axis == Physics.CollidingAxis.Y) {
                            var pos = nonstatic.position;
                            pos.Y -= result.fixValue;

                            nonstatic.position = pos;

                            nonstatic.velocity.Y = 0;


                        }




                    }
                }
            }



            rigidBodies.Clear();

        }
    }
}
