using MatrixEngine.GameObjects.Components.PhysicsComponents;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using Debug = MatrixEngine.System.Debug;
namespace MatrixEngine.Physics {
    public class PhysicsEngine {

        private List<RigidBodyComponent> rigidBodies;

        public System.App app
        {
            get;
            private set;
        }

        public PhysicsEngine(System.App app) {
            this.app = app;
            rigidBodies = new List<RigidBodyComponent>();
        }

        public void AddToFrameComputing(RigidBodyComponent rigidBodyComponent) {

            rigidBodies.Add(rigidBodyComponent);


        }
        public void Update() {


            foreach (var item in rigidBodies) {
                if (!item.isStatic) {
                    //TODO: Fix the goddamn drag shit!
                    var multiplier = 1 - item.velocityDrag;
                    if (1 - multiplier <= 0)
                        multiplier = 0;



                    item.velocity += item.gravity;



                    item.position += item.velocity * app.deltaTime;

                    item.velocity = item.velocity * multiplier;


                    //new Vector2f(item.velocity.X - item.velocityDrag.X * app.deltaTime, item.velocity.Y - item.velocityDrag.Y * app.deltaTime);



                }
            }

            //work

            var watch = new Stopwatch();
            watch.Start();

            //Parallel.For(0, rigidBodies.Count, (i) => {
            //    Parallel.For(0, rigidBodies.Count, (x) => {
            //        if (x > i) {
            //            var objectA = rigidBodies.ElementAt(x);
            //            var objectB = rigidBodies.ElementAt(i);

            //            RigidBodyComponent nonstatic;
            //            RigidBodyComponent @static;

            //            if ((objectA.isStatic && objectB.isStatic) || ((!objectA.isStatic) && (!objectB.isStatic))) {
            //                return;
            //            }


            //            if (objectA.isStatic) {
            //                @static = objectA;
            //                nonstatic = objectB;

            //            } else {
            //                @static = objectB;
            //                nonstatic = objectA;
            //            }


            //            var result = nonstatic.rect.GetCollidingFixFromB(@static.rect);

            //            if (result.axis == Physics.CollidingAxis.X) {
            //                var pos = nonstatic.position;
            //                pos.X -= result.fixValue;

            //                nonstatic.position = pos;

            //                nonstatic.velocity.X = 0;

            //            }
            //            if (result.axis == Physics.CollidingAxis.Y) {
            //                var pos = nonstatic.position;
            //                pos.Y -= result.fixValue;

            //                nonstatic.position = pos;

            //                nonstatic.velocity.Y = 0;


            //            }
            //        }

            //    });
            //});

            var static_list = rigidBodies.Where(r => r.isStatic).ToArray();
            var non_static_list = rigidBodies.Where(r => !r.isStatic).ToArray();


            foreach (var @static in static_list) {
                foreach (var nonstatic in non_static_list) {
                    var result = nonstatic.rect.GetCollidingFixFromB(@static.rect);
                    if (result.axis == Physics.CollidingAxis.None) {
                        continue;
                    }
                    if (result.axis == Physics.CollidingAxis.X) {
                        var pos = nonstatic.position;
                        pos.X -= result.fixValue;

                        nonstatic.position = pos;

                        nonstatic.velocity.X = 0;

                    }
                    else if (result.axis == Physics.CollidingAxis.Y) {
                        var pos = nonstatic.position;
                        pos.Y -= result.fixValue;

                        nonstatic.position = pos;

                        nonstatic.velocity.Y = 0;


                    }
                }
            }
            watch.Stop();
            Debug.Log("ph: " + watch.Elapsed.TotalSeconds.ToString());


            rigidBodies.Clear();

        }
    }
}
